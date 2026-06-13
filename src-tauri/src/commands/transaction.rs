use rusqlite::{params, Transaction};
use serde::{Deserialize, Serialize};
use tauri::State;

use crate::db::DbState;

/// Satu item dalam keranjang/transaksi.
/// `product_id` null untuk item percetakan/jasa (item_name dipakai sebagai label).
#[derive(Debug, Serialize, Deserialize)]
pub struct CartItem {
    pub product_id: Option<i64>,
    pub item_name: String,
    pub quantity: i64,
    pub unit_price: f64,
    pub subtotal: f64,
}

/// Payload untuk memproses transaksi dari Kasir/POS.
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionInput {
    /// PENJUALAN / PERCETAKAN / JASA — untuk transaksi mix (Modul POS Fase 2)
    /// selalu dikirim sebagai "PENJUALAN".
    pub transaction_type: String,
    pub total_amount: f64,
    pub paid_amount: f64,
    pub change_amount: f64,
    pub notes: Option<String>,
    pub items: Vec<CartItem>,
}

/// Hasil setelah transaksi berhasil disimpan.
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionResult {
    pub id: i64,
    pub invoice_no: String,
}

/// Ringkasan transaksi untuk riwayat/dashboard.
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionSummary {
    pub id: i64,
    pub invoice_no: String,
    pub transaction_type: String,
    pub total_amount: f64,
    pub paid_amount: f64,
    pub change_amount: f64,
    pub transaction_date: String,
    pub notes: Option<String>,
}

/// Generate nomor invoice unik: INV-YYYYMMDD-XXXX (XXXX = counter harian).
fn generate_invoice_no(tx: &Transaction) -> Result<String, String> {
    let today: String = tx
        .query_row("SELECT date('now')", [], |row| row.get(0))
        .map_err(|e| e.to_string())?;

    let date_compact = today.replace('-', "");

    let count: i64 = tx
        .query_row(
            "SELECT COUNT(*) FROM transactions WHERE date(transaction_date) = date('now')",
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    Ok(format!("INV-{}-{:04}", date_compact, count + 1))
}

/// Proses transaksi POS: simpan ke `transactions` + `transaction_items`,
/// kurangi stok produk, dan catat ke `stock_movements` (type OUT).
/// Seluruh operasi dijalankan dalam satu DB transaction (atomic) —
/// jika salah satu langkah gagal, semua dibatalkan (rollback).
#[tauri::command]
pub fn process_transaction(
    state: State<DbState>,
    payload: TransactionInput,
) -> Result<TransactionResult, String> {
    if payload.items.is_empty() {
        return Err("Keranjang masih kosong".to_string());
    }

    let mut conn = state.0.lock().map_err(|e| e.to_string())?;
    let tx = conn.transaction().map_err(|e| e.to_string())?;

    let invoice_no = generate_invoice_no(&tx)?;

    tx.execute(
        "INSERT INTO transactions
            (invoice_no, type, total_amount, paid_amount, change_amount, transaction_date, notes)
         VALUES (?1, ?2, ?3, ?4, ?5, datetime('now'), ?6)",
        params![
            invoice_no,
            payload.transaction_type,
            payload.total_amount,
            payload.paid_amount,
            payload.change_amount,
            payload.notes,
        ],
    )
    .map_err(|e| e.to_string())?;

    let transaction_id = tx.last_insert_rowid();

    for item in &payload.items {
        tx.execute(
            "INSERT INTO transaction_items
                (transaction_id, product_id, item_name, quantity, unit_price, subtotal)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                transaction_id,
                item.product_id,
                item.item_name,
                item.quantity,
                item.unit_price,
                item.subtotal,
            ],
        )
        .map_err(|e| e.to_string())?;

        // Hanya kurangi stok jika item terkait produk fisik (product_id ada).
        if let Some(product_id) = item.product_id {
            // Cek stok cukup
            let current_stock: i64 = tx
                .query_row(
                    "SELECT stock FROM products WHERE id = ?1",
                    params![product_id],
                    |row| row.get(0),
                )
                .map_err(|e| format!("Produk tidak ditemukan: {e}"))?;

            if current_stock < item.quantity {
                return Err(format!(
                    "Stok tidak cukup untuk \"{}\" (sisa {}, butuh {})",
                    item.item_name, current_stock, item.quantity
                ));
            }

            tx.execute(
                "UPDATE products SET stock = stock - ?1 WHERE id = ?2",
                params![item.quantity, product_id],
            )
            .map_err(|e| e.to_string())?;

            tx.execute(
                "INSERT INTO stock_movements
                    (product_id, type, quantity, reference, date, notes)
                 VALUES (?1, 'OUT', ?2, ?3, datetime('now'), 'Penjualan via Kasir/POS')",
                params![product_id, item.quantity, invoice_no],
            )
            .map_err(|e| e.to_string())?;
        }
    }

    tx.commit().map_err(|e| e.to_string())?;

    Ok(TransactionResult {
        id: transaction_id,
        invoice_no,
    })
}

/// Ambil riwayat transaksi terbaru (untuk dashboard / riwayat kasir),
/// dibatasi `limit` baris, urut terbaru dulu.
#[tauri::command]
pub fn get_recent_transactions(
    state: State<DbState>,
    limit: i64,
) -> Result<Vec<TransactionSummary>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare(
            "SELECT id, invoice_no, type, total_amount, paid_amount, change_amount, transaction_date, notes
             FROM transactions
             ORDER BY transaction_date DESC, id DESC
             LIMIT ?1",
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map(params![limit], |row| {
            Ok(TransactionSummary {
                id: row.get(0)?,
                invoice_no: row.get(1)?,
                transaction_type: row.get(2)?,
                total_amount: row.get(3)?,
                paid_amount: row.get(4)?,
                change_amount: row.get(5)?,
                transaction_date: row.get(6)?,
                notes: row.get(7)?,
            })
        })
        .map_err(|e| e.to_string())?;

    rows.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())
}
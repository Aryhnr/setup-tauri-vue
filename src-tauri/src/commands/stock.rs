use rusqlite::params;
use serde::{Deserialize, Serialize};
use tauri::State;

use crate::db::DbState;

/// Payload untuk mencatat barang masuk dari supplier.
#[derive(Debug, Serialize, Deserialize)]
pub struct StockInInput {
    pub product_id: i64,
    pub quantity: i64,
    pub supplier_id: Option<i64>,
    pub notes: Option<String>,
    pub reference: Option<String>,
}

/// Satu baris riwayat pergerakan stok.
#[derive(Debug, Serialize, Deserialize)]
pub struct StockMovement {
    pub id: i64,
    pub product_id: i64,
    pub product_name: String,
    pub movement_type: String,
    pub quantity: i64,
    pub reference: Option<String>,
    pub supplier_name: Option<String>,
    pub date: String,
    pub notes: Option<String>,
}

/// Catat barang masuk dari supplier: tambah stok produk + catat ke stock_movements.
/// Operasi dijalankan atomic dalam satu DB transaction.
#[tauri::command]
pub fn add_stock_in(state: State<DbState>, payload: StockInInput) -> Result<(), String> {
    if payload.quantity <= 0 {
        return Err("Jumlah barang masuk harus lebih dari 0".to_string());
    }

    let mut conn = state.0.lock().map_err(|e| e.to_string())?;
    let tx = conn.transaction().map_err(|e| e.to_string())?;

    // Pastikan produk ada
    let exists: bool = tx
        .query_row(
            "SELECT COUNT(*) FROM products WHERE id = ?1",
            params![payload.product_id],
            |row| row.get::<_, i64>(0),
        )
        .map(|count| count > 0)
        .map_err(|e| format!("Produk tidak ditemukan: {e}"))?;

    if !exists {
        return Err(format!("Produk dengan id {} tidak ditemukan", payload.product_id));
    }

    // Tambah stok produk
    tx.execute(
        "UPDATE products SET stock = stock + ?1 WHERE id = ?2",
        params![payload.quantity, payload.product_id],
    )
    .map_err(|e| e.to_string())?;

    // Catat ke stock_movements
    tx.execute(
        "INSERT INTO stock_movements (product_id, type, quantity, reference, supplier_id, date, notes)
         VALUES (?1, 'IN', ?2, ?3, ?4, datetime('now'), ?5)",
        params![
            payload.product_id,
            payload.quantity,
            payload.reference,
            payload.supplier_id,
            payload.notes,
        ],
    )
    .map_err(|e| e.to_string())?;

    tx.commit().map_err(|e| e.to_string())?;

    Ok(())
}

/// Ambil riwayat pergerakan stok, opsional filter per produk.
/// Urut terbaru dulu, dibatasi `limit` baris.
#[tauri::command]
pub fn get_stock_movements(
    state: State<DbState>,
    product_id: Option<i64>,
    limit: Option<i64>,
) -> Result<Vec<StockMovement>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;

    let lim = limit.unwrap_or(100);

    let (query, use_filter) = if product_id.is_some() {
        (
            "SELECT sm.id, sm.product_id, p.name, sm.type, sm.quantity,
                    sm.reference, s.name AS supplier_name, sm.date, sm.notes
             FROM stock_movements sm
             JOIN products p ON p.id = sm.product_id
             LEFT JOIN suppliers s ON s.id = sm.supplier_id
             WHERE sm.product_id = ?1
             ORDER BY sm.date DESC, sm.id DESC
             LIMIT ?2".to_string(),
            true,
        )
    } else {
        (
            "SELECT sm.id, sm.product_id, p.name, sm.type, sm.quantity,
                    sm.reference, s.name AS supplier_name, sm.date, sm.notes
             FROM stock_movements sm
             JOIN products p ON p.id = sm.product_id
             LEFT JOIN suppliers s ON s.id = sm.supplier_id
             ORDER BY sm.date DESC, sm.id DESC
             LIMIT ?1".to_string(),
            false,
        )
    };

    let map_row = |row: &rusqlite::Row| -> rusqlite::Result<StockMovement> {
        Ok(StockMovement {
            id: row.get(0)?,
            product_id: row.get(1)?,
            product_name: row.get(2)?,
            movement_type: row.get(3)?,
            quantity: row.get(4)?,
            reference: row.get(5)?,
            supplier_name: row.get(6)?,
            date: row.get(7)?,
            notes: row.get(8)?,
        })
    };

    let mut stmt = conn.prepare(&query).map_err(|e| e.to_string())?;

    let movements = if use_filter {
        let pid = product_id.unwrap();
        let rows = stmt
            .query_map(params![pid, lim], map_row)
            .map_err(|e| e.to_string())?;
        rows.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())?
    } else {
        let rows = stmt
            .query_map(params![lim], map_row)
            .map_err(|e| e.to_string())?;
        rows.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())?
    };

    Ok(movements)
}

/// Ambil daftar produk dengan stok menipis (stock <= min_stock).
/// Dipakai oleh composable useNotification untuk cek saat startup/transaksi.
#[tauri::command]
pub fn get_low_stock_products(state: State<DbState>) -> Result<Vec<serde_json::Value>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare(
            "SELECT id, name, stock, min_stock, unit
             FROM products
             WHERE stock <= min_stock AND min_stock > 0
             ORDER BY stock ASC, name ASC",
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(serde_json::json!({
                "id": row.get::<_, i64>(0)?,
                "name": row.get::<_, String>(1)?,
                "stock": row.get::<_, i64>(2)?,
                "min_stock": row.get::<_, i64>(3)?,
                "unit": row.get::<_, String>(4)?,
            }))
        })
        .map_err(|e| e.to_string())?;

    rows.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())
}
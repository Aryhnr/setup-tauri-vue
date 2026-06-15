use rusqlite::params;
use serde::{Deserialize, Serialize};
use tauri::State;

use crate::db::DbState;

#[derive(Debug, Serialize, Deserialize)]
pub struct CashierReportInput {
    pub modal_awal: f64,
    pub uang_aktual: f64,
    pub catatan: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CashierReportSummary {
    pub id: i64,
    pub tanggal: String,
    pub modal_awal: f64,
    pub total_penjualan: f64,
    pub total_percetakan: f64,
    pub total_jasa: f64,
    pub total_pemasukan: f64,
    pub total_seharusnya: f64,
    pub uang_aktual: f64,
    pub selisih: f64,
    pub catatan: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DailySummary {
    pub total_penjualan: f64,
    pub total_percetakan: f64,
    pub total_jasa: f64,
    pub total_pemasukan: f64,
    pub jumlah_transaksi: i64,
}

/// Ambil ringkasan transaksi hari ini untuk keperluan tutup kasir.
#[tauri::command]
pub fn get_daily_summary(state: State<DbState>) -> Result<DailySummary, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;

    let penjualan: f64 = conn
        .query_row(
            "SELECT COALESCE(SUM(total_amount), 0) FROM transactions
             WHERE date(transaction_date) = date('now') AND type = 'PENJUALAN'",
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    let percetakan: f64 = conn
        .query_row(
            "SELECT COALESCE(SUM(total_price), 0) FROM print_orders
             WHERE date(created_at) = date('now') AND status = 'DIBAYAR'",
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    let jasa: f64 = conn
        .query_row(
            "SELECT COALESCE(SUM(sell_price), 0) FROM services
             WHERE date(transaction_date) = date('now')",
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    let jumlah: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM transactions
             WHERE date(transaction_date) = date('now')",
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    Ok(DailySummary {
        total_penjualan: penjualan,
        total_percetakan: percetakan,
        total_jasa: jasa,
        total_pemasukan: penjualan + percetakan + jasa,
        jumlah_transaksi: jumlah,
    })
}

/// Simpan rekap kasir harian.
#[tauri::command]
pub fn save_cashier_report(
    state: State<DbState>,
    payload: CashierReportInput,
) -> Result<i64, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;

    // Hitung ulang dari DB supaya akurat
    let penjualan: f64 = conn
        .query_row(
            "SELECT COALESCE(SUM(total_amount), 0) FROM transactions
             WHERE date(transaction_date) = date('now') AND type = 'PENJUALAN'",
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    let percetakan: f64 = conn
        .query_row(
            "SELECT COALESCE(SUM(total_price), 0) FROM print_orders
             WHERE date(created_at) = date('now') AND status = 'DIBAYAR'",
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    let jasa: f64 = conn
        .query_row(
            "SELECT COALESCE(SUM(sell_price), 0) FROM services
             WHERE date(transaction_date) = date('now')",
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    let total_pemasukan = penjualan + percetakan + jasa;
    let total_seharusnya = payload.modal_awal + total_pemasukan;
    let selisih = payload.uang_aktual - total_seharusnya;

    conn.execute(
        "INSERT INTO cashier_reports
            (tanggal, modal_awal, total_penjualan, total_percetakan, total_jasa,
             total_pemasukan, total_seharusnya, uang_aktual, selisih, catatan, created_at)
         VALUES (date('now'), ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, datetime('now'))",
        params![
            payload.modal_awal,
            penjualan,
            percetakan,
            jasa,
            total_pemasukan,
            total_seharusnya,
            payload.uang_aktual,
            selisih,
            payload.catatan,
        ],
    )
    .map_err(|e| e.to_string())?;

    Ok(conn.last_insert_rowid())
}

/// Ambil riwayat rekap kasir, dibatasi limit baris terbaru.
#[tauri::command]
pub fn get_cashier_reports(
    state: State<DbState>,
    limit: Option<i64>,
) -> Result<Vec<CashierReportSummary>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let lim = limit.unwrap_or(30);

    let mut stmt = conn
        .prepare(
            "SELECT id, tanggal, modal_awal, total_penjualan, total_percetakan, total_jasa,
                    total_pemasukan, total_seharusnya, uang_aktual, selisih, catatan, created_at
             FROM cashier_reports
             ORDER BY created_at DESC
             LIMIT ?1",
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map(params![lim], |row| {
            Ok(CashierReportSummary {
                id: row.get(0)?,
                tanggal: row.get(1)?,
                modal_awal: row.get(2)?,
                total_penjualan: row.get(3)?,
                total_percetakan: row.get(4)?,
                total_jasa: row.get(5)?,
                total_pemasukan: row.get(6)?,
                total_seharusnya: row.get(7)?,
                uang_aktual: row.get(8)?,
                selisih: row.get(9)?,
                catatan: row.get(10)?,
                created_at: row.get(11)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(rows)
}
use rusqlite::params;
use serde::{Deserialize, Serialize};
use tauri::State;

use crate::db::DbState;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceTransaction {
    pub id: Option<i64>,
    /// TOKEN_PLN / TOPUP / PULSA / PAKET_DATA
    pub service_type: String,
    pub description: Option<String>,
    /// No. meter / No. HP pelanggan
    pub customer_info: Option<String>,
    pub sell_price: f64,
    pub buy_price: f64,
    pub profit: f64,
    pub transaction_date: Option<String>,
}

fn map_row(row: &rusqlite::Row) -> rusqlite::Result<ServiceTransaction> {
    Ok(ServiceTransaction {
        id: row.get(0)?,
        service_type: row.get(1)?,
        description: row.get(2)?,
        customer_info: row.get(3)?,
        sell_price: row.get(4)?,
        buy_price: row.get(5)?,
        profit: row.get(6)?,
        transaction_date: row.get(7)?,
    })
}

const SELECT_COLUMNS: &str =
    "id, service_type, description, customer_info, sell_price, buy_price, profit, transaction_date";

/// Ambil seluruh transaksi jasa digital, opsional filter berdasarkan
/// service_type. Urut terbaru dulu.
#[tauri::command]
pub fn get_services(state: State<DbState>, service_type: Option<String>) -> Result<Vec<ServiceTransaction>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;

    let query = if service_type.is_some() {
        format!("SELECT {SELECT_COLUMNS} FROM services WHERE service_type = ?1 ORDER BY transaction_date DESC, id DESC")
    } else {
        format!("SELECT {SELECT_COLUMNS} FROM services ORDER BY transaction_date DESC, id DESC")
    };

    let mut stmt = conn.prepare(&query).map_err(|e| e.to_string())?;

    let services = if let Some(t) = service_type {
        let rows = stmt.query_map(params![t], map_row).map_err(|e| e.to_string())?;
        rows.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())?
    } else {
        let rows = stmt.query_map([], map_row).map_err(|e| e.to_string())?;
        rows.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())?
    };

    Ok(services)
}

/// Tambah transaksi jasa digital baru. `profit` dihitung otomatis
/// di backend sebagai (sell_price - buy_price) untuk konsistensi data,
/// meskipun frontend juga mengirim nilainya.
#[tauri::command]
pub fn add_service(state: State<DbState>, service: ServiceTransaction) -> Result<i64, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;

    let profit = service.sell_price - service.buy_price;

    conn.execute(
        "INSERT INTO services
            (service_type, description, customer_info, sell_price, buy_price, profit, transaction_date)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, datetime('now'))",
        params![
            service.service_type,
            service.description,
            service.customer_info,
            service.sell_price,
            service.buy_price,
            profit,
        ],
    )
    .map_err(|e| e.to_string())?;

    Ok(conn.last_insert_rowid())
}

/// Hapus transaksi jasa digital.
#[tauri::command]
pub fn delete_service(state: State<DbState>, id: i64) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;

    conn.execute("DELETE FROM services WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;

    Ok(())
}

/// Ringkasan profit jasa digital hari ini (untuk dashboard / laporan ringkas).
#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceSummary {
    pub total_sell: f64,
    pub total_profit: f64,
    pub count: i64,
}

#[tauri::command]
pub fn get_service_summary_today(state: State<DbState>) -> Result<ServiceSummary, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;

    conn.query_row(
        "SELECT
            COALESCE(SUM(sell_price), 0),
            COALESCE(SUM(profit), 0),
            COUNT(*)
         FROM services
         WHERE date(transaction_date) = date('now')",
        [],
        |row| {
            Ok(ServiceSummary {
                total_sell: row.get(0)?,
                total_profit: row.get(1)?,
                count: row.get(2)?,
            })
        },
    )
    .map_err(|e| e.to_string())
}
use rusqlite::{params, Transaction};
use serde::{Deserialize, Serialize};
use tauri::State;

use crate::db::DbState;

#[derive(Debug, Serialize, Deserialize)]
pub struct PrintOrder {
    pub id: Option<i64>,
    pub order_no: Option<String>,
    /// PRINT_DOC / PRINT_FOTO / FOTOCOPY
    pub service_type: String,
    pub paper_size: Option<String>,
    /// BERWARNA / HITAM_PUTIH
    pub color_mode: Option<String>,
    pub pages: i64,
    pub copies: i64,
    pub price_per_page: f64,
    pub total_price: f64,
    /// ANTRIAN / DIPROSES / SELESAI / DIBAYAR
    pub status: String,
    pub created_at: Option<String>,
}

fn map_row(row: &rusqlite::Row) -> rusqlite::Result<PrintOrder> {
    Ok(PrintOrder {
        id: row.get(0)?,
        order_no: row.get(1)?,
        service_type: row.get(2)?,
        paper_size: row.get(3)?,
        color_mode: row.get(4)?,
        pages: row.get(5)?,
        copies: row.get(6)?,
        price_per_page: row.get(7)?,
        total_price: row.get(8)?,
        status: row.get(9)?,
        created_at: row.get(10)?,
    })
}

const SELECT_COLUMNS: &str = "id, order_no, service_type, paper_size, color_mode, pages, copies, price_per_page, total_price, status, created_at";

/// Generate nomor order unik: PRC-YYYYMMDD-XXXX (counter harian).
fn generate_order_no(tx: &Transaction) -> Result<String, String> {
    let today: String = tx
        .query_row("SELECT date('now')", [], |row| row.get(0))
        .map_err(|e| e.to_string())?;

    let date_compact = today.replace('-', "");

    let count: i64 = tx
        .query_row(
            "SELECT COUNT(*) FROM print_orders WHERE date(created_at) = date('now')",
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    Ok(format!("PRC-{}-{:04}", date_compact, count + 1))
}

/// Ambil seluruh order percetakan, opsional filter berdasarkan status.
/// Urut terbaru dulu.
#[tauri::command]
pub fn get_print_orders(state: State<DbState>, status: Option<String>) -> Result<Vec<PrintOrder>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;

    let query = if status.is_some() {
        format!("SELECT {SELECT_COLUMNS} FROM print_orders WHERE status = ?1 ORDER BY created_at DESC, id DESC")
    } else {
        format!("SELECT {SELECT_COLUMNS} FROM print_orders ORDER BY created_at DESC, id DESC")
    };

    let mut stmt = conn.prepare(&query).map_err(|e| e.to_string())?;

    let orders = if let Some(s) = status {
        let rows = stmt.query_map(params![s], map_row).map_err(|e| e.to_string())?;
        rows.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())?
    } else {
        let rows = stmt.query_map([], map_row).map_err(|e| e.to_string())?;
        rows.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())?
    };

    Ok(orders)
}

/// Tambah order percetakan baru. Status awal selalu "ANTRIAN".
/// `order_no` di-generate otomatis (PRC-YYYYMMDD-XXXX).
#[tauri::command]
pub fn add_print_order(state: State<DbState>, order: PrintOrder) -> Result<i64, String> {
    let mut conn = state.0.lock().map_err(|e| e.to_string())?;
    let tx = conn.transaction().map_err(|e| e.to_string())?;

    let order_no = generate_order_no(&tx)?;

    tx.execute(
        "INSERT INTO print_orders
            (order_no, service_type, paper_size, color_mode, pages, copies, price_per_page, total_price, status, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, 'ANTRIAN', datetime('now'))",
        params![
            order_no,
            order.service_type,
            order.paper_size,
            order.color_mode,
            order.pages,
            order.copies,
            order.price_per_page,
            order.total_price,
        ],
    )
    .map_err(|e| e.to_string())?;

    let id = tx.last_insert_rowid();
    tx.commit().map_err(|e| e.to_string())?;

    Ok(id)
}

/// Update detail order percetakan (tidak mengubah status/order_no).
#[tauri::command]
pub fn update_print_order(state: State<DbState>, id: i64, order: PrintOrder) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;

    conn.execute(
        "UPDATE print_orders SET
            service_type = ?1,
            paper_size = ?2,
            color_mode = ?3,
            pages = ?4,
            copies = ?5,
            price_per_page = ?6,
            total_price = ?7
         WHERE id = ?8",
        params![
            order.service_type,
            order.paper_size,
            order.color_mode,
            order.pages,
            order.copies,
            order.price_per_page,
            order.total_price,
            id,
        ],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

/// Update status order: ANTRIAN -> DIPROSES -> SELESAI -> DIBAYAR.
#[tauri::command]
pub fn update_print_order_status(state: State<DbState>, id: i64, status: String) -> Result<(), String> {
    let valid_statuses = ["ANTRIAN", "DIPROSES", "SELESAI", "DIBAYAR"];
    if !valid_statuses.contains(&status.as_str()) {
        return Err(format!("Status tidak valid: {status}"));
    }

    let conn = state.0.lock().map_err(|e| e.to_string())?;

    conn.execute(
        "UPDATE print_orders SET status = ?1 WHERE id = ?2",
        params![status, id],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

/// Hapus order percetakan.
#[tauri::command]
pub fn delete_print_order(state: State<DbState>, id: i64) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;

    conn.execute("DELETE FROM print_orders WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;

    Ok(())
}
use rusqlite::params;
use serde::{Deserialize, Serialize};
use tauri::State;

use crate::db::DbState;

#[derive(Debug, Serialize, Deserialize)]
pub struct Supplier {
    pub id: Option<i64>,
    pub name: String,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub notes: Option<String>,
}

/// Ambil seluruh supplier, opsional filter berdasarkan nama.
#[tauri::command]
pub fn get_suppliers(
    state: State<DbState>,
    keyword: Option<String>,
) -> Result<Vec<Supplier>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;

    let kw = keyword.unwrap_or_default();
    let trimmed = kw.trim();

    let (query, use_filter) = if trimmed.is_empty() {
        (
            "SELECT id, name, phone, address, notes FROM suppliers ORDER BY name ASC".to_string(),
            false,
        )
    } else {
        (
            "SELECT id, name, phone, address, notes FROM suppliers WHERE name LIKE ?1 ORDER BY name ASC".to_string(),
            true,
        )
    };

    let mut stmt = conn.prepare(&query).map_err(|e| e.to_string())?;

    let map_row = |row: &rusqlite::Row| -> rusqlite::Result<Supplier> {
        Ok(Supplier {
            id: row.get(0)?,
            name: row.get(1)?,
            phone: row.get(2)?,
            address: row.get(3)?,
            notes: row.get(4)?,
        })
    };

    let suppliers = if use_filter {
        let like = format!("%{trimmed}%");
        let rows = stmt
            .query_map(params![like], map_row)
            .map_err(|e| e.to_string())?;
        rows.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())?
    } else {
        let rows = stmt.query_map([], map_row).map_err(|e| e.to_string())?;
        rows.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())?
    };

    Ok(suppliers)
}

/// Tambah supplier baru.
#[tauri::command]
pub fn add_supplier(state: State<DbState>, supplier: Supplier) -> Result<i64, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT INTO suppliers (name, phone, address, notes) VALUES (?1, ?2, ?3, ?4)",
        params![supplier.name, supplier.phone, supplier.address, supplier.notes],
    )
    .map_err(|e| e.to_string())?;

    Ok(conn.last_insert_rowid())
}

/// Update supplier berdasarkan id.
#[tauri::command]
pub fn update_supplier(
    state: State<DbState>,
    id: i64,
    supplier: Supplier,
) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;

    conn.execute(
        "UPDATE suppliers SET name = ?1, phone = ?2, address = ?3, notes = ?4 WHERE id = ?5",
        params![supplier.name, supplier.phone, supplier.address, supplier.notes, id],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

/// Hapus supplier berdasarkan id.
#[tauri::command]
pub fn delete_supplier(state: State<DbState>, id: i64) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;

    conn.execute("DELETE FROM suppliers WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;

    Ok(())
}
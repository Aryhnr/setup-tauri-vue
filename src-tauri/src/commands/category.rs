use rusqlite::params;
use serde::{Deserialize, Serialize};
use tauri::State;

use crate::db::DbState;

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    pub id: Option<i64>,
    pub name: String,
}

/// Ambil seluruh kategori, urut berdasarkan nama.
#[tauri::command]
pub fn get_categories(state: State<DbState>) -> Result<Vec<Category>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare("SELECT id, name FROM categories ORDER BY name ASC")
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(Category {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })
        .map_err(|e| e.to_string())?;

    rows.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())
}

/// Tambah kategori baru.
#[tauri::command]
pub fn add_category(state: State<DbState>, name: String) -> Result<i64, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;

    conn.execute("INSERT INTO categories (name) VALUES (?1)", params![name])
        .map_err(|e| e.to_string())?;

    Ok(conn.last_insert_rowid())
}

/// Hapus kategori berdasarkan id.
#[tauri::command]
pub fn delete_category(state: State<DbState>, id: i64) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;

    conn.execute("DELETE FROM categories WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;

    Ok(())
}
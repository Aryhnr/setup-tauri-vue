use rusqlite::params;
use serde::{Deserialize, Serialize};
use tauri::State;

use crate::db::DbState;

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub id: Option<i64>,
    pub barcode: Option<String>,
    pub name: String,
    pub category_id: Option<i64>,
    pub buy_price: f64,
    pub sell_price: f64,
    pub stock: i64,
    pub min_stock: i64,
    pub unit: String,
    pub supplier_id: Option<i64>,
    pub created_at: Option<String>,
    pub category_name: Option<String>,
    pub supplier_name: Option<String>,
}

/// Ambil seluruh produk, opsional filter berdasarkan nama atau barcode.
#[tauri::command]
pub fn get_products(state: State<DbState>, keyword: Option<String>) -> Result<Vec<Product>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;

    let base_query = "
        SELECT
            p.id, p.barcode, p.name, p.category_id, p.buy_price, p.sell_price,
            p.stock, p.min_stock, p.unit, p.supplier_id, p.created_at,
            c.name AS category_name,
            s.name AS supplier_name
        FROM products p
        LEFT JOIN categories c ON c.id = p.category_id
        LEFT JOIN suppliers s ON s.id = p.supplier_id
    ";

    let kw = keyword.unwrap_or_default();
    let trimmed = kw.trim();

    let (query, like_param) = if trimmed.is_empty() {
        (format!("{base_query} ORDER BY p.name ASC"), None)
    } else {
        (
            format!("{base_query} WHERE p.name LIKE ?1 OR p.barcode LIKE ?1 ORDER BY p.name ASC"),
            Some(format!("%{trimmed}%")),
        )
    };

    let mut stmt = conn.prepare(&query).map_err(|e| e.to_string())?;

    let map_row = |row: &rusqlite::Row| -> rusqlite::Result<Product> {
        Ok(Product {
            id: row.get(0)?,
            barcode: row.get(1)?,
            name: row.get(2)?,
            category_id: row.get(3)?,
            buy_price: row.get(4)?,
            sell_price: row.get(5)?,
            stock: row.get(6)?,
            min_stock: row.get(7)?,
            unit: row.get(8)?,
            supplier_id: row.get(9)?,
            created_at: row.get(10)?,
            category_name: row.get(11)?,
            supplier_name: row.get(12)?,
        })
    };

    let products = if let Some(like) = like_param {
        let rows = stmt
            .query_map(params![like], map_row)
            .map_err(|e| e.to_string())?;
        rows.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())?
    } else {
        let rows = stmt.query_map([], map_row).map_err(|e| e.to_string())?;
        rows.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())?
    };

    Ok(products)
}

/// Tambah produk baru.
#[tauri::command]
pub fn add_product(state: State<DbState>, product: Product) -> Result<i64, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT INTO products
            (barcode, name, category_id, buy_price, sell_price, stock, min_stock, unit, supplier_id, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, datetime('now'))",
        params![
            product.barcode,
            product.name,
            product.category_id,
            product.buy_price,
            product.sell_price,
            product.stock,
            product.min_stock,
            product.unit,
            product.supplier_id,
        ],
    )
    .map_err(|e| e.to_string())?;

    Ok(conn.last_insert_rowid())
}

/// Update produk berdasarkan id.
#[tauri::command]
pub fn update_product(state: State<DbState>, id: i64, product: Product) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;

    conn.execute(
        "UPDATE products SET
            barcode = ?1,
            name = ?2,
            category_id = ?3,
            buy_price = ?4,
            sell_price = ?5,
            stock = ?6,
            min_stock = ?7,
            unit = ?8,
            supplier_id = ?9
         WHERE id = ?10",
        params![
            product.barcode,
            product.name,
            product.category_id,
            product.buy_price,
            product.sell_price,
            product.stock,
            product.min_stock,
            product.unit,
            product.supplier_id,
            id,
        ],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

/// Hapus produk berdasarkan id.
#[tauri::command]
pub fn delete_product(state: State<DbState>, id: i64) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;

    conn.execute("DELETE FROM products WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;

    Ok(())
}

/// Cari produk berdasarkan barcode (untuk Modul POS nanti).
#[tauri::command]
pub fn find_product_by_barcode(state: State<DbState>, barcode: String) -> Result<Option<Product>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare(
            "SELECT
                p.id, p.barcode, p.name, p.category_id, p.buy_price, p.sell_price,
                p.stock, p.min_stock, p.unit, p.supplier_id, p.created_at,
                c.name AS category_name,
                s.name AS supplier_name
             FROM products p
             LEFT JOIN categories c ON c.id = p.category_id
             LEFT JOIN suppliers s ON s.id = p.supplier_id
             WHERE p.barcode = ?1
             LIMIT 1",
        )
        .map_err(|e| e.to_string())?;

    let mut rows = stmt
        .query_map(params![barcode], |row| {
            Ok(Product {
                id: row.get(0)?,
                barcode: row.get(1)?,
                name: row.get(2)?,
                category_id: row.get(3)?,
                buy_price: row.get(4)?,
                sell_price: row.get(5)?,
                stock: row.get(6)?,
                min_stock: row.get(7)?,
                unit: row.get(8)?,
                supplier_id: row.get(9)?,
                created_at: row.get(10)?,
                category_name: row.get(11)?,
                supplier_name: row.get(12)?,
            })
        })
        .map_err(|e| e.to_string())?;

    match rows.next() {
        Some(r) => Ok(Some(r.map_err(|e| e.to_string())?)),
        None => Ok(None),
    }
}
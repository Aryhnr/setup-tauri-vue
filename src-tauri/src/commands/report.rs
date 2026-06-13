use rusqlite::params;
use serde::{Deserialize, Serialize};
use tauri::State;

use crate::db::DbState;

/// Satu titik data pendapatan harian (untuk grafik garis/batang).
#[derive(Debug, Serialize, Deserialize)]
pub struct RevenuePoint {
    pub date: String,
    pub total: f64,
    pub count: i64,
}

/// Ringkasan keseluruhan untuk satu rentang tanggal.
#[derive(Debug, Serialize, Deserialize)]
pub struct RevenueSummary {
    pub total_revenue: f64,
    pub total_transactions: i64,
    pub points: Vec<RevenuePoint>,
}

/// Ambil laporan pendapatan harian dari tabel `transactions` (Modul POS),
/// dalam rentang tanggal `start_date` s.d. `end_date` (format YYYY-MM-DD,
/// inklusif kedua ujung).
#[tauri::command]
pub fn get_revenue_report(
    state: State<DbState>,
    start_date: String,
    end_date: String,
) -> Result<RevenueSummary, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare(
            "SELECT date(transaction_date) as d, COALESCE(SUM(total_amount), 0), COUNT(*)
             FROM transactions
             WHERE date(transaction_date) BETWEEN ?1 AND ?2
             GROUP BY d
             ORDER BY d ASC",
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map(params![start_date, end_date], |row| {
            Ok(RevenuePoint {
                date: row.get(0)?,
                total: row.get(1)?,
                count: row.get(2)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let points = rows.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())?;

    let total_revenue = points.iter().map(|p| p.total).sum();
    let total_transactions = points.iter().map(|p| p.count).sum();

    Ok(RevenueSummary {
        total_revenue,
        total_transactions,
        points,
    })
}

/// Satu baris pada laporan produk terlaris / kurang laku.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProductSalesRow {
    pub product_id: Option<i64>,
    pub name: String,
    pub total_quantity: i64,
    pub total_revenue: f64,
}

/// Ambil produk terlaris (atau kurang laku jika `ascending = true`),
/// berdasarkan total quantity terjual dalam rentang tanggal.
/// Hanya menghitung item dengan `product_id` (barang fisik, bukan
/// percetakan/jasa).
#[tauri::command]
pub fn get_top_products(
    state: State<DbState>,
    start_date: String,
    end_date: String,
    limit: i64,
    ascending: bool,
) -> Result<Vec<ProductSalesRow>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;

    let order = if ascending { "ASC" } else { "DESC" };

    let query = format!(
        "SELECT ti.product_id, ti.item_name, SUM(ti.quantity), SUM(ti.subtotal)
         FROM transaction_items ti
         JOIN transactions t ON t.id = ti.transaction_id
         WHERE ti.product_id IS NOT NULL
           AND date(t.transaction_date) BETWEEN ?1 AND ?2
         GROUP BY ti.product_id, ti.item_name
         ORDER BY SUM(ti.quantity) {order}
         LIMIT ?3"
    );

    let mut stmt = conn.prepare(&query).map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map(params![start_date, end_date, limit], |row| {
            Ok(ProductSalesRow {
                product_id: row.get(0)?,
                name: row.get(1)?,
                total_quantity: row.get(2)?,
                total_revenue: row.get(3)?,
            })
        })
        .map_err(|e| e.to_string())?;

    rows.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())
}

/// Satu baris pada laporan per kategori produk.
#[derive(Debug, Serialize, Deserialize)]
pub struct CategorySalesRow {
    pub category_name: String,
    pub total_quantity: i64,
    pub total_revenue: f64,
}

/// Ambil laporan penjualan per kategori produk dalam rentang tanggal.
#[tauri::command]
pub fn get_category_report(
    state: State<DbState>,
    start_date: String,
    end_date: String,
) -> Result<Vec<CategorySalesRow>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare(
            "SELECT COALESCE(c.name, 'Tanpa Kategori'), SUM(ti.quantity), SUM(ti.subtotal)
             FROM transaction_items ti
             JOIN transactions t ON t.id = ti.transaction_id
             LEFT JOIN products p ON p.id = ti.product_id
             LEFT JOIN categories c ON c.id = p.category_id
             WHERE ti.product_id IS NOT NULL
               AND date(t.transaction_date) BETWEEN ?1 AND ?2
             GROUP BY c.name
             ORDER BY SUM(ti.subtotal) DESC",
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map(params![start_date, end_date], |row| {
            Ok(CategorySalesRow {
                category_name: row.get(0)?,
                total_quantity: row.get(1)?,
                total_revenue: row.get(2)?,
            })
        })
        .map_err(|e| e.to_string())?;

    rows.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())
}

/// Ringkasan pendapatan per jenis layanan: toko (penjualan barang),
/// percetakan, dan jasa digital. Dipakai untuk tab "Ringkasan" laporan.
#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceTypeBreakdown {
    pub toko: f64,
    pub percetakan: f64,
    pub jasa_digital: f64,
}

#[tauri::command]
pub fn get_service_type_report(
    state: State<DbState>,
    start_date: String,
    end_date: String,
) -> Result<ServiceTypeBreakdown, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;

    // Penjualan barang fisik (transaction_items dengan product_id),
    // dari transaksi tipe PENJUALAN.
    let toko: f64 = conn
        .query_row(
            "SELECT COALESCE(SUM(ti.subtotal), 0)
             FROM transaction_items ti
             JOIN transactions t ON t.id = ti.transaction_id
             WHERE ti.product_id IS NOT NULL
               AND date(t.transaction_date) BETWEEN ?1 AND ?2",
            params![start_date, end_date],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    // Percetakan: dari print_orders dengan status DIBAYAR.
    let percetakan: f64 = conn
        .query_row(
            "SELECT COALESCE(SUM(total_price), 0)
             FROM print_orders
             WHERE status = 'DIBAYAR'
               AND date(created_at) BETWEEN ?1 AND ?2",
            params![start_date, end_date],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    // Jasa digital: dari tabel services.
    let jasa_digital: f64 = conn
        .query_row(
            "SELECT COALESCE(SUM(sell_price), 0)
             FROM services
             WHERE date(transaction_date) BETWEEN ?1 AND ?2",
            params![start_date, end_date],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    Ok(ServiceTypeBreakdown {
        toko,
        percetakan,
        jasa_digital,
    })
}

/// Satu baris produk dengan stok menipis/habis (untuk laporan stok).
#[derive(Debug, Serialize, Deserialize)]
pub struct StockAlertRow {
    pub id: i64,
    pub name: String,
    pub stock: i64,
    pub min_stock: i64,
    pub unit: String,
}

/// Ambil daftar produk dengan stok habis (stock = 0) dan stok menipis
/// (0 < stock <= min_stock), untuk laporan stok.
#[tauri::command]
pub fn get_stock_alert_report(state: State<DbState>) -> Result<Vec<StockAlertRow>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare(
            "SELECT id, name, stock, min_stock, unit
             FROM products
             WHERE stock <= min_stock
             ORDER BY stock ASC, name ASC",
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(StockAlertRow {
                id: row.get(0)?,
                name: row.get(1)?,
                stock: row.get(2)?,
                min_stock: row.get(3)?,
                unit: row.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?;

    rows.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())
}
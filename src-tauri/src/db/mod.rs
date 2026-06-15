use rusqlite::Connection;
use std::sync::Mutex;
use tauri::Manager;

/// Wrapper state yang dipegang Tauri, menyimpan koneksi SQLite.
/// Mutex digunakan karena rusqlite::Connection tidak Sync.
pub struct DbState(pub Mutex<Connection>);

/// Schema lengkap sesuai PRD section 6.
const SCHEMA_SQL: &str = r#"
CREATE TABLE IF NOT EXISTS categories (
    id   INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS suppliers (
    id      INTEGER PRIMARY KEY AUTOINCREMENT,
    name    TEXT NOT NULL,
    phone   TEXT,
    address TEXT,
    notes   TEXT
);

CREATE TABLE IF NOT EXISTS products (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    barcode     TEXT UNIQUE,
    name        TEXT NOT NULL,
    category_id INTEGER REFERENCES categories(id) ON DELETE SET NULL,
    buy_price   REAL NOT NULL DEFAULT 0,
    sell_price  REAL NOT NULL DEFAULT 0,
    stock       INTEGER NOT NULL DEFAULT 0,
    min_stock   INTEGER NOT NULL DEFAULT 0,
    unit        TEXT NOT NULL DEFAULT 'pcs',
    supplier_id INTEGER REFERENCES suppliers(id) ON DELETE SET NULL,
    created_at  DATETIME NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_products_name ON products(name);
CREATE INDEX IF NOT EXISTS idx_products_barcode ON products(barcode);

CREATE TABLE IF NOT EXISTS transactions (
    id               INTEGER PRIMARY KEY AUTOINCREMENT,
    invoice_no       TEXT UNIQUE NOT NULL,
    type             TEXT NOT NULL CHECK (type IN ('PENJUALAN', 'PERCETAKAN', 'JASA')),
    total_amount     REAL NOT NULL DEFAULT 0,
    paid_amount      REAL NOT NULL DEFAULT 0,
    change_amount    REAL NOT NULL DEFAULT 0,
    transaction_date DATETIME NOT NULL DEFAULT (datetime('now')),
    notes            TEXT
);

CREATE TABLE IF NOT EXISTS transaction_items (
    id             INTEGER PRIMARY KEY AUTOINCREMENT,
    transaction_id INTEGER NOT NULL REFERENCES transactions(id) ON DELETE CASCADE,
    product_id     INTEGER REFERENCES products(id) ON DELETE SET NULL,
    item_name      TEXT NOT NULL,
    quantity       INTEGER NOT NULL DEFAULT 1,
    unit_price     REAL NOT NULL DEFAULT 0,
    subtotal       REAL NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS stock_movements (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    product_id  INTEGER NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    type        TEXT NOT NULL CHECK (type IN ('IN', 'OUT')),
    quantity    INTEGER NOT NULL,
    reference   TEXT,
    supplier_id INTEGER REFERENCES suppliers(id) ON DELETE SET NULL,
    date        DATETIME NOT NULL DEFAULT (datetime('now')),
    notes       TEXT
);

CREATE TABLE IF NOT EXISTS print_orders (
    id             INTEGER PRIMARY KEY AUTOINCREMENT,
    order_no       TEXT UNIQUE NOT NULL,
    service_type   TEXT NOT NULL CHECK (service_type IN ('PRINT_DOC', 'PRINT_FOTO', 'FOTOCOPY')),
    paper_size     TEXT,
    color_mode     TEXT CHECK (color_mode IN ('BERWARNA', 'HITAM_PUTIH')),
    pages          INTEGER NOT NULL DEFAULT 1,
    copies         INTEGER NOT NULL DEFAULT 1,
    price_per_page REAL NOT NULL DEFAULT 0,
    total_price    REAL NOT NULL DEFAULT 0,
    status         TEXT NOT NULL DEFAULT 'ANTRIAN' CHECK (status IN ('ANTRIAN', 'DIPROSES', 'SELESAI', 'DIBAYAR')),
    created_at     DATETIME NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS services (
    id                INTEGER PRIMARY KEY AUTOINCREMENT,
    service_type      TEXT NOT NULL CHECK (service_type IN ('TOKEN_PLN', 'TOPUP', 'PULSA', 'PAKET_DATA')),
    description       TEXT,
    customer_info     TEXT,
    sell_price        REAL NOT NULL DEFAULT 0,
    buy_price         REAL NOT NULL DEFAULT 0,
    profit            REAL NOT NULL DEFAULT 0,
    transaction_date  DATETIME NOT NULL DEFAULT (datetime('now'))
);
CREATE TABLE IF NOT EXISTS cashier_reports (
    id               INTEGER PRIMARY KEY AUTOINCREMENT,
    tanggal          DATE NOT NULL,
    modal_awal       REAL NOT NULL DEFAULT 0,
    total_penjualan  REAL NOT NULL DEFAULT 0,
    total_percetakan REAL NOT NULL DEFAULT 0,
    total_jasa       REAL NOT NULL DEFAULT 0,
    total_pemasukan  REAL NOT NULL DEFAULT 0,
    total_seharusnya REAL NOT NULL DEFAULT 0,
    uang_aktual      REAL NOT NULL DEFAULT 0,
    selisih          REAL NOT NULL DEFAULT 0,
    catatan          TEXT,
    created_at       DATETIME NOT NULL DEFAULT (datetime('now'))
);

INSERT OR IGNORE INTO categories (name) VALUES
    ('Makanan & Minuman'),
    ('Rokok'),
    ('Sembako'),
    ('Peralatan Rumah'),
    ('Lainnya');

CREATE TABLE IF NOT EXISTS debts (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    name       TEXT NOT NULL,
    phone      TEXT,
    amount     REAL NOT NULL DEFAULT 0,
    paid       REAL NOT NULL DEFAULT 0,
    remaining  REAL NOT NULL DEFAULT 0,
    notes      TEXT,
    due_date   DATE,
    status     TEXT NOT NULL DEFAULT 'AKTIF' CHECK (status IN ('AKTIF', 'LUNAS')),
    created_at DATETIME NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS debt_payments (
    id       INTEGER PRIMARY KEY AUTOINCREMENT,
    debt_id  INTEGER NOT NULL REFERENCES debts(id) ON DELETE CASCADE,
    amount   REAL NOT NULL DEFAULT 0,
    date     DATETIME NOT NULL DEFAULT (datetime('now')),
    notes    TEXT
);
"#;

/// Membuka koneksi SQLite di app data dir (toko.db) dan menjalankan schema.
/// Dipanggil sekali saat aplikasi start, hasilnya dimasukkan ke Tauri state.
pub fn init_db(app: &tauri::App) -> Connection {
    let app_dir = app
        .path()
        .app_data_dir()
        .expect("gagal mendapatkan app data dir");

    std::fs::create_dir_all(&app_dir).expect("gagal membuat folder app data dir");

    let db_path = app_dir.join("toko.db");
    let conn = Connection::open(db_path).expect("gagal membuka database SQLite");

    conn.execute_batch(SCHEMA_SQL)
        .expect("gagal menjalankan schema migration");

    conn
}
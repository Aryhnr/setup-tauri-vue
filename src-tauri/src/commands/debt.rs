use rusqlite::params;
use serde::{Deserialize, Serialize};
use tauri::State;

use crate::db::DbState;

#[derive(Debug, Serialize, Deserialize)]
pub struct Debt {
    pub id: i64,
    pub name: String,
    pub phone: Option<String>,
    pub amount: f64,
    pub paid: f64,
    pub remaining: f64,
    pub notes: Option<String>,
    pub due_date: Option<String>,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DebtInput {
    pub name: String,
    pub phone: Option<String>,
    pub amount: f64,
    pub notes: Option<String>,
    pub due_date: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DebtPayment {
    pub id: i64,
    pub debt_id: i64,
    pub amount: f64,
    pub date: String,
    pub notes: Option<String>,
}

/// Ambil semua hutang, opsional filter status: AKTIF / LUNAS / null (semua).
#[tauri::command]
pub fn get_debts(
    state: State<DbState>,
    status: Option<String>,
) -> Result<Vec<Debt>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;

    let (query, use_filter) = if status.is_some() {
        (
            "SELECT id, name, phone, amount, paid, remaining, notes, due_date, status, created_at
             FROM debts WHERE status = ?1 ORDER BY created_at DESC".to_string(),
            true,
        )
    } else {
        (
            "SELECT id, name, phone, amount, paid, remaining, notes, due_date, status, created_at
             FROM debts ORDER BY created_at DESC".to_string(),
            false,
        )
    };

    let map_row = |row: &rusqlite::Row| -> rusqlite::Result<Debt> {
        Ok(Debt {
            id: row.get(0)?,
            name: row.get(1)?,
            phone: row.get(2)?,
            amount: row.get(3)?,
            paid: row.get(4)?,
            remaining: row.get(5)?,
            notes: row.get(6)?,
            due_date: row.get(7)?,
            status: row.get(8)?,
            created_at: row.get(9)?,
        })
    };

    let mut stmt = conn.prepare(&query).map_err(|e| e.to_string())?;

    let rows = if use_filter {
        stmt.query_map(params![status.unwrap()], map_row)
            .map_err(|e| e.to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?
    } else {
        stmt.query_map([], map_row)
            .map_err(|e| e.to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?
    };

    Ok(rows)
}

/// Tambah hutang baru.
#[tauri::command]
pub fn add_debt(
    state: State<DbState>,
    payload: DebtInput,
) -> Result<i64, String> {
    if payload.amount <= 0.0 {
        return Err("Nominal hutang harus lebih dari 0".to_string());
    }

    let conn = state.0.lock().map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT INTO debts (name, phone, amount, paid, remaining, notes, due_date, status, created_at)
         VALUES (?1, ?2, ?3, 0, ?3, ?4, ?5, 'AKTIF', datetime('now'))",
        params![
            payload.name,
            payload.phone,
            payload.amount,
            payload.notes,
            payload.due_date,
        ],
    )
    .map_err(|e| e.to_string())?;

    Ok(conn.last_insert_rowid())
}

/// Catat pembayaran cicilan hutang.
/// Otomatis update paid, remaining, dan status (LUNAS jika remaining <= 0).
#[tauri::command]
pub fn add_debt_payment(
    state: State<DbState>,
    debt_id: i64,
    amount: f64,
    notes: Option<String>,
) -> Result<(), String> {
    if amount <= 0.0 {
        return Err("Jumlah bayar harus lebih dari 0".to_string());
    }

    let mut conn = state.0.lock().map_err(|e| e.to_string())?;
    let tx = conn.transaction().map_err(|e| e.to_string())?;

    // Cek hutang ada dan ambil sisa
    let (current_paid, total_amount): (f64, f64) = tx
        .query_row(
            "SELECT paid, amount FROM debts WHERE id = ?1",
            params![debt_id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .map_err(|e| format!("Hutang tidak ditemukan: {e}"))?;

    let new_paid = current_paid + amount;
    let new_remaining = (total_amount - new_paid).max(0.0);
    let new_status = if new_remaining <= 0.0 { "LUNAS" } else { "AKTIF" };

    // Update hutang
    tx.execute(
        "UPDATE debts SET paid = ?1, remaining = ?2, status = ?3 WHERE id = ?4",
        params![new_paid, new_remaining, new_status, debt_id],
    )
    .map_err(|e| e.to_string())?;

    // Catat pembayaran
    tx.execute(
        "INSERT INTO debt_payments (debt_id, amount, date, notes)
         VALUES (?1, ?2, datetime('now'), ?3)",
        params![debt_id, amount, notes],
    )
    .map_err(|e| e.to_string())?;

    tx.commit().map_err(|e| e.to_string())?;
    Ok(())
}

/// Ambil riwayat pembayaran per hutang.
#[tauri::command]
pub fn get_debt_payments(
    state: State<DbState>,
    debt_id: i64,
) -> Result<Vec<DebtPayment>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare(
            "SELECT id, debt_id, amount, date, notes
             FROM debt_payments WHERE debt_id = ?1
             ORDER BY date DESC",
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map(params![debt_id], |row| {
            Ok(DebtPayment {
                id: row.get(0)?,
                debt_id: row.get(1)?,
                amount: row.get(2)?,
                date: row.get(3)?,
                notes: row.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(rows)
}

/// Hapus hutang beserta riwayat pembayarannya.
#[tauri::command]
pub fn delete_debt(state: State<DbState>, id: i64) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM debts WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

/// Ambil ringkasan hutang untuk dashboard:
/// total hutang aktif, total nominal, total sisa.
#[tauri::command]
pub fn get_debt_summary(state: State<DbState>) -> Result<serde_json::Value, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;

    let row = conn
        .query_row(
            "SELECT COUNT(*), COALESCE(SUM(amount),0), COALESCE(SUM(remaining),0)
             FROM debts WHERE status = 'AKTIF'",
            [],
            |row| Ok((row.get::<_, i64>(0)?, row.get::<_, f64>(1)?, row.get::<_, f64>(2)?)),
        )
        .map_err(|e| e.to_string())?;

    Ok(serde_json::json!({
        "count": row.0,
        "total_amount": row.1,
        "total_remaining": row.2,
    }))
}
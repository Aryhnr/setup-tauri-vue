use std::path::PathBuf;
use tauri::Manager;

/// Backup database ke folder tujuan yang dipilih user.
/// Nama file: toko_backup_YYYYMMDD_HHMMSS.db
/// Return: path lengkap file backup yang dibuat.
#[tauri::command]
pub fn backup_database(app: tauri::AppHandle, dest_dir: String) -> Result<String, String> {
    let dest_path = PathBuf::from(&dest_dir);

    if !dest_path.exists() {
        return Err(format!("Folder tujuan tidak ditemukan: {dest_dir}"));
    }

    let db_path = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?
        .join("toko.db");

    if !db_path.exists() {
        return Err("File database tidak ditemukan".to_string());
    }

    // Format timestamp: YYYYMMDD_HHMMSS
    let now = chrono::Local::now();
    let filename = format!("toko_backup_{}.db", now.format("%Y%m%d_%H%M%S"));
    let backup_file = dest_path.join(&filename);

    std::fs::copy(&db_path, &backup_file)
        .map_err(|e| format!("Gagal menyalin database: {e}"))?;

    Ok(backup_file.to_string_lossy().to_string())
}

/// Restore database dari file backup yang dipilih user.
/// Database aktif diganti dengan file backup (current db di-rename dulu sebagai safety).
/// App perlu di-restart setelah restore.
#[tauri::command]
pub fn restore_database(app: tauri::AppHandle, backup_file: String) -> Result<(), String> {
    let src = PathBuf::from(&backup_file);

    if !src.exists() {
        return Err(format!("File backup tidak ditemukan: {backup_file}"));
    }

    // Validasi ekstensi
    match src.extension().and_then(|e| e.to_str()) {
        Some("db") => {}
        _ => return Err("File backup harus berekstensi .db".to_string()),
    }

    let app_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;

    let db_path = app_dir.join("toko.db");

    // Rename database aktif ke .bak dulu sebagai safety net
    let bak_path = app_dir.join("toko.db.bak");
    if db_path.exists() {
        std::fs::rename(&db_path, &bak_path)
            .map_err(|e| format!("Gagal memindahkan database aktif: {e}"))?;
    }

    // Copy file backup menjadi database aktif
    if let Err(e) = std::fs::copy(&src, &db_path) {
        // Rollback: kembalikan .bak jika copy gagal
        if bak_path.exists() {
            let _ = std::fs::rename(&bak_path, &db_path);
        }
        return Err(format!("Gagal merestore database: {e}"));
    }

    // Hapus .bak jika restore sukses
    if bak_path.exists() {
        let _ = std::fs::remove_file(&bak_path);
    }

    Ok(())
}

/// Ambil path folder AppData yang digunakan aplikasi.
/// Dipakai frontend untuk menampilkan lokasi database.
#[tauri::command]
pub fn get_app_data_dir(app: tauri::AppHandle) -> Result<String, String> {
    let dir = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;

    Ok(dir.to_string_lossy().to_string())
}
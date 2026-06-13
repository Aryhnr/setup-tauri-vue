mod commands;
mod db;

use db::DbState;
use std::sync::Mutex;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let conn = db::init_db(app);
            app.manage(DbState(Mutex::new(conn)));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::product::get_products,
            commands::product::add_product,
            commands::product::update_product,
            commands::product::delete_product,
            commands::product::find_product_by_barcode,
            commands::category::get_categories,
            commands::category::add_category,
            commands::category::delete_category,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
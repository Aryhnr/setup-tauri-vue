mod commands;
mod db;

use db::DbState;
use std::sync::Mutex;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
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
            commands::transaction::process_transaction,
            commands::transaction::get_recent_transactions,
            commands::print_order::get_print_orders,
            commands::print_order::add_print_order,
            commands::print_order::update_print_order,
            commands::print_order::update_print_order_status,
            commands::print_order::delete_print_order,
            commands::service::get_services,
            commands::service::add_service,
            commands::service::delete_service,
            commands::service::get_service_summary_today,
            commands::report::get_revenue_report,
            commands::report::get_top_products,
            commands::report::get_category_report,
            commands::report::get_service_type_report,
            commands::report::get_stock_alert_report,
            // Fase 5 — Supplier
            commands::supplier::get_suppliers,
            commands::supplier::add_supplier,
            commands::supplier::update_supplier,
            commands::supplier::delete_supplier,
            // Fase 5 — Stock
            commands::stock::add_stock_in,
            commands::stock::get_stock_movements,
            commands::stock::get_low_stock_products,
            // Fase 5 — Backup
            commands::backup::backup_database,
            commands::backup::restore_database,
            commands::backup::get_app_data_dir,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
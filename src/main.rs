#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

mod search;
mod indexer;
mod workspaces;
mod hotkey;
mod window_manager;

fn main() {
    env_logger::init();

    tauri::Builder::default()
        .setup(|app| {
            if let Err(e) = hotkey::register_global_hotkey(app.handle()) {
                log::error!("Failed to register hotkey: {}", e);
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            search::search_query,
            workspaces::load_workspaces,
            workspaces::save_workspace,
            window_manager::list_windows,
            window_manager::minimize_all_windows,
            window_manager::restore_all_windows,
            window_manager::focus_window,
            window_manager::arrange_windows_layout,
            window_manager::save_current_layout,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
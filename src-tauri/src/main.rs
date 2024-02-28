// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
mod state;

use state::{AppState, ServiceAccess};
use tauri::{AppHandle, Manager, State};

#[tauri::command]
fn greet(app_handle: AppHandle, name: &str, state: &str) -> String {
    // Should handle errors instead of unwrapping here
    app_handle
        .db(|db| database::add_item(name, state, db))
        .unwrap();

    let items = app_handle.db(|db| database::get_all(db)).unwrap();

    let items_string = serde_json::to_string(&items).unwrap();

    items_string
}

#[tauri::command]
fn remove(app_handle: AppHandle) {
    app_handle.db_mut(|db| {
        database::remove_database(&app_handle);
    })
}

#[tauri::command]
fn clear(app_handle: AppHandle) {
    app_handle.db_mut(|db| {
        database::clear_all(db).unwrap();
    });
}

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            db: Default::default(),
        })
        .invoke_handler(tauri::generate_handler![greet, clear, remove])
        .setup(|app| {
            let handle = app.handle();

            let app_state: State<AppState> = handle.state();
            let db =
                database::initialize_database(&handle).expect("Database initialize should succeed");
            *app_state.db.lock().unwrap() = Some(db);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

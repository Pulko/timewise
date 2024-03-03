// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
mod item;
mod state;

use state::{AppState, ServiceAccess};
use tauri::{AppHandle, Manager, State};

#[tauri::command]
fn add(app_handle: AppHandle, name: &str, state: &str) {
    if let Err(e) = app_handle.db(|db| database::add_item(name, state, db)) {
        eprintln!("Error adding item: {}", e);
    };
}

#[tauri::command]
fn remove(app_handle: AppHandle) {
    app_handle.db_mut(|_db| {
        database::remove_database(&app_handle);
    })
}

#[tauri::command]
fn fetch(app_handle: AppHandle) -> String {
    match app_handle.db(|db| database::get_all(db)) {
        Ok(items) => {
            let ser_items = serde_json::to_string(&items).unwrap();

            ser_items
        }
        Err(e) => {
            eprintln!("Error fetching items: {}", e);
            String::from("[]")
        }
    }
}

#[tauri::command]
fn fetch_by_state(app_handle: AppHandle, state: &str) -> String {
    match app_handle.db(|db| database::get_by_state(db, state)) {
        Ok(items) => {
            let ser_items = serde_json::to_string(&items).unwrap();

            ser_items
        }
        Err(e) => {
            eprintln!("Error fetching unfinished items: {}", e);
            String::from("[]")
        }
    }
}

#[tauri::command]
fn remove_item(app_handle: AppHandle, title: &str) {
    match app_handle.db_mut(|db| database::remove_item(title, db)) {
        Ok(_) => (),
        Err(e) => eprintln!("Error removing item: {}", e),
    }
}

#[tauri::command]
fn clear(app_handle: AppHandle) {
    match app_handle.db_mut(|db| database::clear_all(db)) {
        Ok(_) => (),
        Err(e) => eprintln!("Error clearing items: {}", e),
    }
}

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            db: Default::default(),
        })
        .invoke_handler(tauri::generate_handler![
            add,
            clear,
            remove,
            fetch,
            fetch_by_state,
            remove_item
        ])
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

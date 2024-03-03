use rusqlite::{named_params, Connection};
use std::fs;
use tauri::AppHandle;

use crate::item::Item;

const CURRENT_DB_VERSION: u32 = 1;
const DB_PATH: &str = "MyApp.sqlite";

pub fn remove_database(app_handle: &AppHandle) {
    let app_dir = app_handle
        .path_resolver()
        .app_data_dir()
        .expect("The app data directory should exist.");
    let sqlite_path = app_dir.join(DB_PATH);
    fs::remove_file(sqlite_path).expect("The database file should be removed.");
}

pub fn initialize_database(app_handle: &AppHandle) -> Result<Connection, rusqlite::Error> {
    let app_dir = app_handle
        .path_resolver()
        .app_data_dir()
        .expect("The app data directory should exist.");
    fs::create_dir_all(&app_dir).expect("The app data directory should be created.");
    let sqlite_path = app_dir.join(DB_PATH);

    let mut db = Connection::open(sqlite_path)?;

    let mut user_pragma = db.prepare("PRAGMA user_version")?;
    let existing_user_version: u32 = user_pragma.query_row([], |row| Ok(row.get(0)?))?;
    drop(user_pragma);

    upgrade_database_if_needed(&mut db, existing_user_version)?;

    Ok(db)
}

pub fn upgrade_database_if_needed(
    db: &mut Connection,
    existing_version: u32,
) -> Result<(), rusqlite::Error> {
    if existing_version < CURRENT_DB_VERSION {
        db.pragma_update(None, "journal_mode", "WAL")?;

        let tx = db.transaction()?;

        tx.pragma_update(None, "user_version", CURRENT_DB_VERSION)?;

        tx.execute_batch(
            "
      CREATE TABLE items (
        title TEXT NOT NULL,
        state TEXT NOT NULL
      );",
        )?;

        tx.commit()?;
    }

    Ok(())
}

pub fn add_item(title: &str, state: &str, db: &Connection) -> Result<(), rusqlite::Error> {
    if let Ok(existing_item) = get_item(title, db) {
        if existing_item.state != state {
            update_item_state(title, state, db)?;
        }

        return Ok(());
    }

    let mut statement = db.prepare("INSERT INTO items (title, state) VALUES (@title, @state)")?;

    statement.execute(named_params! {
        "@title": title,
        "@state": state
    })?;

    Ok(())
}

pub fn remove_item(title: &str, db: &Connection) -> Result<(), rusqlite::Error> {
    let mut statement = db.prepare("DELETE FROM items WHERE title = @title")?;

    statement.execute(named_params! {
        "@title": title
    })?;

    Ok(())
}

pub fn update_item_state(title: &str, state: &str, db: &Connection) -> Result<(), rusqlite::Error> {
    let mut statement = db.prepare("UPDATE items SET state = @state WHERE title = @title")?;

    statement.execute(named_params! {
        "@title": title,
        "@state": state
    })?;

    Ok(())
}

pub fn clear_all(db: &Connection) -> Result<(), rusqlite::Error> {
    db.execute_batch("DELETE FROM items")?;

    Ok(())
}

pub fn get_item(title: &str, db: &Connection) -> Result<Item, rusqlite::Error> {
    let mut statement = db.prepare("SELECT * FROM items WHERE title = @title")?;
    let mut rows = statement.query(named_params! {
        "@title": title
    })?;

    if let Some(row) = rows.next()? {
        return Ok(get_item_from_row(&row)?);
    }

    Err(rusqlite::Error::QueryReturnedNoRows)
}

pub fn get_all(db: &Connection) -> Result<Vec<Item>, rusqlite::Error> {
    let mut statement = db.prepare("SELECT * FROM items")?;
    let mut rows = statement.query([])?;
    let mut items = Vec::new();
    while let Some(row) = rows.next()? {
        items.push(get_item_from_row(&row)?);
    }

    Ok(items)
}

pub fn get_by_state(db: &Connection, state: &str) -> Result<Vec<Item>, rusqlite::Error> {
    let mut statement = db.prepare("SELECT * FROM items WHERE state = @state")?;
    let mut rows = statement.query(named_params! {
        "@state": state
    })?;
    let mut items = Vec::new();
    while let Some(row) = rows.next()? {
        items.push(get_item_from_row(&row)?);
    }

    Ok(items)
}

fn get_item_from_row(row: &rusqlite::Row) -> Result<Item, rusqlite::Error> {
    // descoped to updated for new versions of schema
    let title: String = row.get("title")?;
    let state: String = row.get("state")?;

    Ok(Item { title, state })
}

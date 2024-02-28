use rusqlite::{named_params, Connection};
use serde::{ser::SerializeStruct, Serialize};
use std::fs;
use tauri::AppHandle;

const CURRENT_DB_VERSION: u32 = 1;

pub fn remove_database(app_handle: &AppHandle) {
    let app_dir = app_handle
        .path_resolver()
        .app_data_dir()
        .expect("The app data directory should exist.");
    let sqlite_path = app_dir.join("MyApp.sqlite");
    fs::remove_file(sqlite_path).expect("The database file should be removed.");
}

pub fn initialize_database(app_handle: &AppHandle) -> Result<Connection, rusqlite::Error> {
    let app_dir = app_handle
        .path_resolver()
        .app_data_dir()
        .expect("The app data directory should exist.");
    fs::create_dir_all(&app_dir).expect("The app data directory should be created.");
    let sqlite_path = app_dir.join("MyApp.sqlite");

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

pub fn get_all(db: &Connection) -> Result<Vec<Item>, rusqlite::Error> {
    let mut statement = db.prepare("SELECT * FROM items")?;
    let mut rows = statement.query([])?;
    let mut items = Vec::new();
    while let Some(row) = rows.next()? {
        let title: String = row.get("title")?;
        let state: String = row.get("state")?;

        items.push(Item { title, state });
    }

    Ok(items)
}

pub struct Item {
    pub title: String,
    pub state: String,
}

impl Serialize for Item {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Item", 2)?;
        state.serialize_field("title", &self.title)?;
        state.serialize_field("state", &self.state)?;
        state.end()
    }
}

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::named_params;

use crate::types::LogEntry;

pub fn create_table_and_pool() -> Pool<SqliteConnectionManager> {
    // Create a connection manager for SQLite
    let manager = SqliteConnectionManager::file("gpjc_logs.db");

    // Create a pool of SQLite connections
    let pool = r2d2::Pool::new(manager).expect("Failed to create connection pool");

    // Establish a connection and create the table if it doesn't exist
    let conn = pool
        .get()
        .expect("Failed to get a connection from the pool");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS gpjc_logs (
            TRANSACTION_ID INTEGER PRIMARY KEY,
            RESULT INTEGER,
            COMPUTATION_START TEXT,
            COMPUTATION_END TEXT,
            IS_INITIATOR BOOLEAN
        )",
        (),
    )
    .expect("Failed to create gpjc_logs table");

    pool
}

pub fn insert_data(
    pool: &Pool<SqliteConnectionManager>,
    transaction_id: i32,
    result: i32,
    computation_start: String,
    is_initiator: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let conn = pool.get()?;
    conn.execute(
        "INSERT INTO gpjc_logs (TRANSACTION_ID, RESULT, COMPUTATION_START, IS_INITIATOR) VALUES (:TRANSACTION_ID, :RESULT, :COMPUTATION_START, :IS_INITIATOR)",
        named_params! {
            ":TRANSACTION_ID": &transaction_id,
            ":RESULT": &result,
            ":COMPUTATION_START": &computation_start,
            ":IS_INITIATOR": &is_initiator
        },
    )?;
    Ok(())
}

pub fn update_data(
    pool: &Pool<SqliteConnectionManager>,
    transaction_id: i32,
    result: i32,
    computation_end: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let conn = pool.get()?;
    conn.execute(
        "UPDATE gpjc_logs SET RESULT = :RESULT, COMPUTATION_END = :COMPUTATION_END WHERE TRANSACTION_ID = :TRANSACTION_ID",
        named_params! {
            ":RESULT": &result,
            ":COMPUTATION_END": &computation_end,
            ":TRANSACTION_ID": &transaction_id
        },
    )?;
    Ok(())
}

pub fn _select_data(
    pool: &Pool<SqliteConnectionManager>,
    transaction_id: i32,
) -> Result<Option<LogEntry>, Box<dyn std::error::Error>> {
    let conn = pool.get()?;
    let mut stmt = conn.prepare("SELECT * FROM gpjc_logs WHERE TRANSACTION_ID = ?")?;
    let mut rows = stmt.query(&[&transaction_id])?;
    if let Some(row) = rows.next()? {
        let log_entry = LogEntry {
            transaction_id: row.get(0)?,
            result: row.get(1)?,
            computation_start: row.get(2)?,
            computation_end: row.get(3)?,
            is_initiator: row.get(4)?,
        };
        Ok(Some(log_entry))
    } else {
        Ok(None)
    }
}

pub fn select_all_data(
    pool: &Pool<SqliteConnectionManager>,
) -> Result<Vec<LogEntry>, Box<dyn std::error::Error>> {
    let conn = pool.get()?;
    let mut stmt = conn.prepare("SELECT * FROM gpjc_logs")?;
    let rows = stmt.query_map((), |row| {
        Ok(LogEntry {
            transaction_id: row.get(0)?,
            result: row.get(1)?,
            computation_start: row.get(2)?,
            computation_end: row.get(3)?,
            is_initiator: row.get(4)?,
        })
    })?;

    let mut result = Vec::new();
    for row in rows {
        result.push(row?);
    }

    Ok(result)
}

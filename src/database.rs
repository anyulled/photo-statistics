use rusqlite::{Connection, Result};
use serde_json::Value;
use crate::utils::{normalize_focal_length, normalize_white_balance};

const DB_FILE: &str = "photo_stats_cache.db";

pub fn create_tables_if_needed() -> Result<()> {
    let conn = Connection::open(DB_FILE)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS metadata (
            source_file TEXT PRIMARY KEY,
            mod_time REAL,
            DateTimeOriginal TEXT,
            Model TEXT,
            LensModel TEXT,
            ISO TEXT,
            ExposureTime TEXT,
            FNumber TEXT,
            FocalLength TEXT,
            Flash TEXT,
            WhiteBalance TEXT,
            ImageWidth TEXT,
            ImageHeight TEXT,
            FocalLengthIn35mmFormat TEXT
        )",
        [],
    )?;
    Ok(())
}

pub fn insert_metadata(conn: &Connection, file_path: &str, mod_time: f64, metadata: &Value) -> Result<()> {
    conn.execute(
        "INSERT OR REPLACE INTO metadata VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        rusqlite::params![
            file_path,
            mod_time,
            metadata["DateTimeOriginal"].as_str().unwrap_or("N/A"),
            metadata["Model"].as_str().unwrap_or("N/A"),
            metadata["LensModel"].as_str().unwrap_or("N/A"),
            metadata["ISO"].as_str().unwrap_or("N/A"),
            metadata["ExposureTime"].as_str().unwrap_or("N/A"),
            metadata["FNumber"].as_str().unwrap_or("N/A"),
            normalize_focal_length(metadata["FocalLength"].as_str()),  // Ensures proper formatting
            metadata["Flash"].as_str().unwrap_or("N/A"),
            normalize_white_balance(metadata["WhiteBalance"].as_str()),
            metadata["ImageWidth"].as_str().unwrap_or("N/A"),
            metadata["ImageHeight"].as_str().unwrap_or("N/A"),
            metadata["FocalLengthIn35mmFormat"].as_str().unwrap_or("N/A"),
        ],
    )?;
    Ok(())
}

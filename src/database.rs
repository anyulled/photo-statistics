use crate::utils::{normalize_focal_length, normalize_white_balance};
use rusqlite::{Connection, Result};
use serde_json::Value;
use chrono::NaiveDateTime;

pub fn create_tables_if_needed(conn: &Connection) -> Result<()> {
    println!("🔍 Creating tables...");
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
    println!("✅ Table created!");
    Ok(())
}

pub fn insert_metadata(
    conn: &Connection,
    file_path: &str,
    mod_time: f64,
    metadata: &Value,
) -> Result<()> {
    conn.execute(
        "INSERT OR REPLACE INTO metadata VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        rusqlite::params![
            file_path,
            mod_time,
            metadata["DateTimeOriginal"]
                .as_str()
                .map_or("N/A".to_string(), format_date),
            metadata["Model"].as_str().unwrap_or("N/A"),
            metadata["LensModel"].as_str().unwrap_or("N/A"),
            metadata["ISO"].as_str().unwrap_or("N/A"),
            metadata["ExposureTime"].as_str().unwrap_or("N/A"),
            metadata["FNumber"].as_str().unwrap_or("N/A"),
            normalize_focal_length(metadata["FocalLength"].as_str()), // Ensures proper formatting
            metadata["Flash"].as_str().unwrap_or("N/A"),
            normalize_white_balance(metadata["WhiteBalance"].as_str()),
            metadata["ImageWidth"].as_str().unwrap_or("N/A"),
            metadata["ImageHeight"].as_str().unwrap_or("N/A"),
            metadata["FocalLengthIn35mmFormat"]
                .as_str()
                .unwrap_or("N/A"),
        ],
    )?;
    Ok(())
}

pub fn format_date(date: &str) -> String {
    if let Ok(parsed) = NaiveDateTime::parse_from_str(date, "%Y:%m:%d %H:%M:%S") {
        return parsed.format("%Y-%m-%d %H:%M:%S").to_string();
    }
    "N/A".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;
    use serde_json::json;

    fn setup_test_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        create_tables_if_needed(&conn).unwrap();
        conn
    }

    #[test]
    fn test_create_tables() {
        let conn = setup_test_db();
        let tables: i64 = conn
            .query_row(
                "SELECT count(*) FROM sqlite_master WHERE type='table' AND name='metadata'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(tables, 1);
    }

    #[test]
    fn test_insert_metadata() {
        let conn = setup_test_db();
        let metadata = json!({
            "DateTimeOriginal": "2023:06:12 15:30:00",
            "Model": "Canon EOS R5",
            "LensModel": "RF 24-70mm",
            "ISO": "100",
            "ExposureTime": "1/200",
            "FNumber": "2.8",
            "FocalLength": "50 mm",
            "Flash": "Off",
            "WhiteBalance": "Auto",
            "ImageWidth": "8192",
            "ImageHeight": "5464",
            "FocalLengthIn35mmFormat": "50"
        });

        insert_metadata(&conn, "test.jpg", 1234567890.0, &metadata).unwrap();

        let count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM metadata WHERE source_file = 'test.jpg'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_insert_null_metadata() {
        let conn = setup_test_db();
        let metadata = json!({}); // Empty metadata

        insert_metadata(&conn, "test.jpg", 1234567890.0, &metadata).unwrap();

        let result: String = conn
            .query_row(
                "SELECT FocalLength FROM metadata WHERE source_file = 'test.jpg'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(result, "N/A"); // Should be "N/A" instead of empty
    }

    #[test]
    fn test_db_error_handling() {
        let conn = Connection::open_in_memory().unwrap();
        let broken_metadata = json!({ "InvalidColumn": "Test" });

        let result = insert_metadata(&conn, "broken.jpg", 1234567890.0, &broken_metadata);

        assert!(result.is_err(), "Expected error due to invalid column.");
    }

}

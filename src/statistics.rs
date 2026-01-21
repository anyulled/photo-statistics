//! Statistical analysis and reporting.
//!
//! This module generates various statistics from the metadata stored in the database,
//! such as camera models, ISO usage, and more.

use rusqlite::Connection;
use std::collections::HashMap;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Statistics {
    pub photos_per_year: HashMap<String, i32>,
    pub camera_models: HashMap<String, i32>,
    pub lens_models: HashMap<String, i32>,
    pub iso: HashMap<String, i32>,
    pub shutter_speed: HashMap<String, i32>,
    pub aperture: HashMap<String, i32>,
    pub focal_length: HashMap<String, i32>,
    pub white_balance: HashMap<String, i32>,
    pub flash_usage: HashMap<String, i32>,
}

pub fn generate_statistics(connection: &Connection) -> Result<Statistics, anyhow::Error> {
    let mut stats = Statistics {
        photos_per_year: HashMap::new(),
        camera_models: HashMap::new(),
        lens_models: HashMap::new(),
        iso: HashMap::new(),
        shutter_speed: HashMap::new(),
        aperture: HashMap::new(),
        focal_length: HashMap::new(),
        white_balance: HashMap::new(),
        flash_usage: HashMap::new(),
    };

    let queries = vec![
        ("Photos Per Year", "SELECT COALESCE(strftime('%Y', DateTimeOriginal), 'N/A'), COUNT(*) FROM metadata GROUP BY strftime('%Y', DateTimeOriginal)"),
        ("Camera Models", "SELECT COALESCE(Model, 'N/A'), COUNT(*) FROM metadata GROUP BY Model"),
        ("Lens Models", "SELECT COALESCE(LensModel, 'N/A'), COUNT(*) FROM metadata GROUP BY LensModel"),
        ("ISO", "SELECT COALESCE(ISO, 'N/A'), COUNT(*) FROM metadata GROUP BY ISO"),
        ("Shutter Speed", "SELECT COALESCE(ExposureTime, 'N/A'), COUNT(*) FROM metadata GROUP BY ExposureTime"),
        ("Aperture", "SELECT COALESCE(FNumber, 'N/A'), COUNT(*) FROM metadata GROUP BY FNumber"),
        ("Focal Length", "SELECT COALESCE(FocalLength, 'N/A'), COUNT(*) FROM metadata GROUP BY FocalLength"),
        ("White Balance", "SELECT COALESCE(WhiteBalance, 'N/A'), COUNT(*) FROM metadata GROUP BY WhiteBalance"),
        ("Flash Usage", "SELECT COALESCE(Flash, 'N/A'), COUNT(*) FROM metadata GROUP BY Flash"),
    ];

    for (title, query) in queries {
        let mut stmt = connection.prepare(query)?;
        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0).unwrap_or_else(|_| "Unknown".to_string()),
                row.get(1).unwrap_or(0)
            ))
        })?;

        let results: HashMap<String, i32> = rows
            .filter_map(Result::ok)
            .collect();

        match title {
            "Photos Per Year" => stats.photos_per_year = results,
            "Camera Models" => stats.camera_models = results,
            "Lens Models" => stats.lens_models = results,
            "ISO" => stats.iso = results,
            "Shutter Speed" => stats.shutter_speed = results,
            "Aperture" => stats.aperture = results,
            "Focal Length" => stats.focal_length = results,
            "White Balance" => stats.white_balance = results,
            "Flash Usage" => stats.flash_usage = results,
            _ => {},
        }
    }

    Ok(stats)
}

#[cfg(test)]
mod tests {

    use rusqlite::Connection;
    use serde_json::json;
    use std::collections::HashMap;
    use crate::database::{create_tables_if_needed, insert_metadata};
    use crate::statistics::generate_statistics;

    fn setup_test_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        create_tables_if_needed(&conn).unwrap();
        conn
    }

    #[test]
    fn test_photo_count_by_year() {
        let conn = setup_test_db();

        let metadata_2021 = json!({ "DateTimeOriginal": "2021:05:10 14:30:00" });
        let metadata_2022 = json!({ "DateTimeOriginal": "2022:07:22 10:15:00" });

        insert_metadata(&conn, "photo1.jpg", 1234567890.0, &metadata_2021).unwrap();
        insert_metadata(&conn, "photo2.jpg", 1234567890.0, &metadata_2022).unwrap();
        insert_metadata(&conn, "photo3.jpg", 1234567890.0, &metadata_2022).unwrap();

        let mut stmt = conn.prepare("SELECT strftime('%Y', DateTimeOriginal), COUNT(*) FROM metadata GROUP BY strftime('%Y', DateTimeOriginal)").unwrap();
        let counts: HashMap<String, i32> = stmt
            .query_map([], |row| Ok((row.get::<_, String>(0)?, row.get(1)?)))
            .unwrap()
            .filter_map(Result::ok)
            .collect();

        assert_eq!(counts.get("2021"), Some(&1));
        assert_eq!(counts.get("2022"), Some(&2));
    }

    #[test]
    fn test_handle_null_dates() {
        let conn = setup_test_db();

        let metadata_no_date = json!({});
        insert_metadata(&conn, "photo1.jpg", 1234567890.0, &metadata_no_date).unwrap();

        let mut stmt = conn.prepare("SELECT COALESCE(strftime('%Y', DateTimeOriginal), 'N/A'), COUNT(*) FROM metadata GROUP BY strftime('%Y', DateTimeOriginal)").unwrap();
        let counts: HashMap<String, i32> = stmt
            .query_map([], |row| Ok((row.get::<_, String>(0)?, row.get(1)?)))
            .unwrap()
            .filter_map(Result::ok)
            .collect();

        assert_eq!(counts.get("N/A"), Some(&1)); // ✅ Should default to "N/A"
    }

    #[test]
    fn test_empty_statistics() {
        let conn = setup_test_db();
        create_tables_if_needed(&conn).unwrap();
        generate_statistics(&conn);
    }

}

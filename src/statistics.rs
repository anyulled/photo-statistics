use rusqlite::Connection;
use std::collections::HashMap;
use std::println;

pub fn generate_statistics(connection: &Connection) {
    let mut stats = HashMap::new();

    let queries = vec![
        ("Photos Per Year",
         "SELECT COALESCE(strftime('%Y', DateTimeOriginal), 'N/A'), COUNT(*) FROM metadata GROUP BY strftime('%Y', DateTimeOriginal)"),
        ("Camera Models",
         "SELECT COALESCE(Model, 'N/A'), COUNT(*) FROM metadata GROUP BY Model"),
        ("Lens Models",
         "SELECT COALESCE(LensModel, 'N/A'), COUNT(*) FROM metadata GROUP BY LensModel"),
        ("ISO",
         "SELECT COALESCE(ISO, 'N/A'), COUNT(*) FROM metadata GROUP BY ISO"),
        ("Shutter Speed",
         "SELECT COALESCE(ExposureTime, 'N/A'), COUNT(*) FROM metadata GROUP BY ExposureTime"),
        ("Aperture",
         "SELECT COALESCE(FNumber, 'N/A'), COUNT(*) FROM metadata GROUP BY FNumber"),
        ("Focal Length",
         "SELECT COALESCE(FocalLength, 'N/A'), COUNT(*) FROM metadata GROUP BY FocalLength"),
        ("White Balance",
         "SELECT COALESCE(WhiteBalance, 'N/A'), COUNT(*) FROM metadata GROUP BY WhiteBalance"),
        ("Flash Usage",
         "SELECT COALESCE(Flash, 'N/A'), COUNT(*) FROM metadata GROUP BY Flash"),
    ];

    for (title, query) in queries {
        let mut stmt = connection.prepare(query).unwrap();
        let results: HashMap<String, i32> = stmt
            .query_map([], |row| Ok((row.get::<_, String>(0)?, row.get(1)?)))
            .unwrap()
            .filter_map(Result::ok)
            .collect();

        stats.insert(title, results);
    }

    for (title, data) in stats {
        println!("📊 {}:", title);
        for (key, count) in data {
            println!("  {}: {}", key, count);
        }
        println!();
    }
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

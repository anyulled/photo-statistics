use rusqlite::Connection;
use std::collections::HashMap;
use std::println;

pub fn generate_statistics() {
    let conn = Connection::open("photo_stats_cache.db").expect("Failed to open database");

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
        let mut stmt = conn.prepare(query).unwrap();
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

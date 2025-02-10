use rayon::prelude::*;
use std::sync::Mutex;
use std::time::UNIX_EPOCH;
use rusqlite::Connection;
use crate::exiftool::run_exiftool;
use crate::database::insert_metadata;
use std::eprintln;

pub fn process_files_in_parallel(files: Vec<String>) {
    let conn = Mutex::new(Connection::open("photo_stats_cache.db").expect("Failed to open database"));

    files.par_chunks(50).for_each(|chunk| {
        let metadata = run_exiftool(chunk).unwrap_or_default();

        for (file, data) in chunk.iter().zip(metadata.iter()) {
            let mod_time = std::fs::metadata(file).and_then(|m| m.modified()).unwrap_or(UNIX_EPOCH);
            let mod_time_secs = mod_time.duration_since(UNIX_EPOCH).unwrap().as_secs_f64();

            let db_conn = conn.lock().unwrap();
            if let Err(err) = insert_metadata(&db_conn, file, mod_time_secs, data) {
                eprintln!("Error inserting metadata: {:?}", err);
            }
        }
    });
}

use rayon::prelude::*;
use std::sync::Mutex;
use std::time::UNIX_EPOCH;
use rusqlite::Connection;
use crate::config::Config;
use crate::exiftool::run_exiftool;
use crate::database::insert_metadata;

/// Process files in parallel, extracting metadata and storing it in the database
///
/// This function processes files in parallel chunks, using transactions for batch processing
/// to reduce database lock contention.
///
/// # Arguments
///
/// * `files` - A vector of file paths to process
/// * `config` - The application configuration
pub fn process_files_in_parallel(files: Vec<String>, config: &Config) {
    let conn = Mutex::new(
        Connection::open(&config.database_path)
            .expect("Failed to open database")
    );

    files.par_chunks(50).for_each(|chunk| {
        let metadata = match run_exiftool(chunk) {
            Ok(data) => data,
            Err(err) => {
                eprintln!("Error running exiftool: {:?}", err);
                return;
            }
        };

        let mut db_conn = match conn.lock() {
            Ok(conn) => conn,
            Err(err) => {
                eprintln!("Error acquiring database lock: {:?}", err);
                return;
            }
        };

        let transaction = match db_conn.transaction() {
            Ok(tx) => tx,
            Err(err) => {
                eprintln!("Error starting transaction: {:?}", err);
                return;
            }
        };

        for (file, data) in chunk.iter().zip(metadata.iter()) {
            let mod_time = std::fs::metadata(file)
                .and_then(|m| m.modified())
                .unwrap_or(UNIX_EPOCH);

            let mod_time_secs = match mod_time.duration_since(UNIX_EPOCH) {
                Ok(duration) => duration.as_secs_f64(),
                Err(err) => {
                    eprintln!("Error calculating duration: {:?}", err);
                    0.0
                }
            };

            if let Err(err) = insert_metadata(&transaction, file, mod_time_secs, data) {
                eprintln!("Error inserting metadata: {:?}", err);
            }
        }

        if let Err(err) = transaction.commit() {
            eprintln!("Error committing transaction: {:?}", err);
        }
    });
}

#[test]
fn test_worker_parallel_processing() {
    let files = vec!["test1.jpg".to_string(), "test2.jpg".to_string()];
    let config = crate::config::Config::new();
    process_files_in_parallel(files, &config);
}

mod config;
mod database;
mod exiftool;
mod files;
mod statistics;
mod utils;
mod worker;

use config::Config;
use database::create_tables_if_needed;
use files::scan_directory;
use rusqlite::Connection;
use statistics::generate_statistics;
use std::env;
use std::time::Instant;
use worker::process_files_in_parallel;

/// Main function for the photo-statistics application
///
/// This function is the entry point for the application. It:
/// 1. Initializes the logger
/// 2. Gets the directory to process from command-line arguments
/// 3. Opens a connection to the database
/// 4. Creates tables if needed
/// 5. Scans the directory for files
/// 6. Processes the files in parallel
/// 7. Generates statistics
/// 8. Prints the elapsed time
fn main() {
    env_logger::init();

    let start_time = Instant::now();
    let directory = env::args().nth(1).unwrap_or_else(|| ".".to_string());

    println!("📂 Processing directory: {}", directory);

    let config = Config::new();

    let conn = match Connection::open(&config.database_path) {
        Ok(conn) => conn,
        Err(err) => {
            eprintln!("Error opening database: {:?}", err);
            eprintln!("Make sure the database path is valid and accessible.");
            return;
        }
    };

    if let Err(err) = create_tables_if_needed(&conn) {
        eprintln!("Error creating database tables: {:?}", err);
        eprintln!("The application may not function correctly without the required tables.");
    }

    println!("🔍 Scanning directory...");
    let files = scan_directory(&directory);

    println!("📷 Found {} files to process.", files.len());

    if files.is_empty() {
        println!("🚫 No valid images found.");
        return;
    }

    println!("🚀 Processing metadata...");
    process_files_in_parallel(files, &config);

    println!("📊 Generating statistics...");
    generate_statistics(&conn);

    println!("✅ Completed in {:.2?}", start_time.elapsed());
}

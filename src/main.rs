mod database;
mod exiftool;
mod files;
mod statistics;
mod utils;
mod worker;

use database::create_tables_if_needed;
use files::scan_directory;
use rusqlite::Connection;
use statistics::generate_statistics;
use std::env;
use std::time::Instant;
use worker::process_files_in_parallel;
const DB_FILE: &str = "photo_stats_cache.db";

fn main() {
    env_logger::init();

    let start_time = Instant::now();
    let directory = env::args().nth(1).unwrap_or_else(|| ".".to_string());

    println!("📂 Processing directory: {}", directory);

    let conn = Connection::open(DB_FILE);

    create_tables_if_needed(&conn.unwrap()).expect("Failed to create database tables.");

    println!("🔍 Scanning directory...");
    let files = scan_directory(&directory);

    println!("📷 Found {} files to process.", files.len());

    if files.is_empty() {
        println!("🚫 No valid images found.");
        return;
    }

    println!("🚀 Processing metadata...");
    process_files_in_parallel(files);

    println!("📊 Generating statistics...");
    generate_statistics();

    println!("✅ Completed in {:.2?}", start_time.elapsed());
}

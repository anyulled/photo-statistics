use photo_statistics::config::{Cli, Config};
use photo_statistics::database::create_tables_if_needed;
use photo_statistics::files::scan_directory;
use photo_statistics::statistics::generate_statistics;
use photo_statistics::worker::process_files_in_parallel;
use anyhow::{Context, Result};
use clap::Parser;
use rusqlite::Connection;
use std::time::Instant;

fn main() -> Result<()> {
    env_logger::init();

    let start_time = Instant::now();
    let args = Cli::parse();
    
    println!("📂 Processing directory: {}", args.directory);

    let config = Config::from(args);

    let conn = Connection::open(&config.database_path)
        .context("Failed to open database")?;

    create_tables_if_needed(&conn)
        .context("Failed to create database tables")?;

    println!("🔍 Scanning directory...");
    let files = scan_directory(&config.directory);

    println!("📷 Found {} files to process.", files.len());

    if files.is_empty() {
        println!("🚫 No valid images found.");
        return Ok(());
    }

    println!("🚀 Processing metadata...");
    process_files_in_parallel(files, &config)
        .context("Failed to process files")?;

    println!("📊 Generating statistics...");
    println!("📊 Generating statistics...");
    let stats = generate_statistics(&conn)
        .context("Failed to generate statistics")?;

    let print_map = |title: &str, map: &std::collections::HashMap<String, i32>| {
        println!("📊 {}:", title);
        for (key, count) in map {
            println!("  {}: {}", key, count);
        }
        println!();
    };

    print_map("Photos Per Year", &stats.photos_per_year);
    print_map("Camera Models", &stats.camera_models);
    print_map("Lens Models", &stats.lens_models);
    print_map("ISO", &stats.iso);
    print_map("Shutter Speed", &stats.shutter_speed);
    print_map("Aperture", &stats.aperture);
    print_map("Focal Length", &stats.focal_length);
    print_map("White Balance", &stats.white_balance);
    print_map("Flash Usage", &stats.flash_usage);

    println!("✅ Completed in {:.2?}", start_time.elapsed());
    
    Ok(())
}

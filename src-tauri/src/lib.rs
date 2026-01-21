use photo_statistics::statistics::{generate_statistics, Statistics};
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::{State, Manager};

struct AppState {
    conn: Mutex<Connection>,
}

#[tauri::command]
fn get_statistics(state: State<AppState>) -> Result<Statistics, String> {
    let conn = state.conn.lock().map_err(|_| "Failed to lock database connection")?;
    generate_statistics(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
async fn start_scan(directory: String) -> Result<String, String> {
    let path = std::path::Path::new(&directory);
    if !path.exists() {
        return Err(format!("Directory does not exist: {}", directory));
    }
    
    // Create config
    let config = photo_statistics::config::Config {
        directory: directory.clone(),
        database_path: std::path::PathBuf::from("photo_stats_cache.db"),
    };

    let files = photo_statistics::files::scan_directory(&directory);
        
    let count = files.len();
    
    photo_statistics::worker::process_files_in_parallel(files, &config)
        .map_err(|e| e.to_string())?;
        
    Ok(format!("Successfully processed {} photos.", count))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      let db_path = "photo_stats_cache.db";
      // Ensure the database exists or handle error. 
      // For dev, we assume it might exist or we create it.
      // In a real app, we might check app_data_dir.
      let conn = Connection::open(db_path).map_err(|e| e.to_string())?;
      
      // Initialize tables if needed
      photo_statistics::database::create_tables_if_needed(&conn).map_err(|e| e.to_string())?;

      app.manage(AppState {
          conn: Mutex::new(conn),
      });

      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![get_statistics, start_scan])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

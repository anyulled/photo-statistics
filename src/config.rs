use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Directory to scan for photos
    #[arg(default_value = ".")]
    pub directory: String,

    /// Path to the SQLite database
    #[arg(short, long, default_value = "photo_stats_cache.db")]
    pub database: PathBuf,
}

/// Configuration for the photo-statistics application
#[derive(Debug, Clone)]
pub struct Config {
    pub database_path: PathBuf,
    pub directory: String,
}

impl From<Cli> for Config {
    fn from(args: Cli) -> Self {
        Self {
            database_path: args.database,
            directory: args.directory,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_config_mapping() {
        let args = Cli {
            directory: "photos".to_string(),
            database: PathBuf::from("test.db"),
        };
        let config: Config = args.into();
        assert_eq!(config.database_path, Path::new("test.db"));
        assert_eq!(config.directory, "photos");
    }
}
use std::path::PathBuf;

/// Configuration for the photo-statistics application
pub struct Config {
    pub database_path: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            database_path: PathBuf::from("photo_stats_cache.db"),
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new configuration with a custom database path
    ///
    /// This method is provided for future extensibility, allowing users to specify
    /// a custom database path through command-line arguments or configuration files.
    /// Currently not used in the main application, but kept for future use.
    #[allow(dead_code)]
    pub fn with_database_path<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.database_path = path.into();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.database_path, Path::new("photo_stats_cache.db"));
    }

    #[test]
    fn test_custom_database_path() {
        let config = Config::new().with_database_path("custom_db.sqlite");
        assert_eq!(config.database_path, Path::new("custom_db.sqlite"));
    }
}

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
}
use crate::configuration::Configuration;

pub struct ConfigurationLookup {
    config: Configuration,
}

// Default color for a file that doesn't have a valid configuration
// rule.
const DEFAULT_COLOR: &str = "gray";

impl ConfigurationLookup {
    // Constructor for the ConfigurationLookup struct
    pub fn new() -> Self {
        Self {
            config: Configuration::new(),
        }
    }
    // Retrieves the color for the provided file name.
    pub fn get_color(&self, file: &str) -> &str {
        match self.config.get_value(file) {
            Ok(color) => color.unwrap_or(DEFAULT_COLOR),
            Err(_) => {
                println!("Invalid config rule for {}", file);
                DEFAULT_COLOR
            }
        }
    }
}

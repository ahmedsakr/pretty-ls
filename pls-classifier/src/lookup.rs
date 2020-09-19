use crate::configuration::Configuration;

pub struct ConfigurationLookup {
    config: Configuration,
}

impl ConfigurationLookup {
    // Constructor for the ConfigurationLookup struct
    pub fn new() -> Self {
        Self {
            config: Configuration::new(),
        }
    }
    // Retrieves the color for the provided file name.
    pub fn get_color(&self, file: &str) -> String {
        match self.config.get_value(file) {
            Ok(color) => color.unwrap_or(self.get_default_color()),
            Err(_) => {
                println!("Invalid config rule for {}", file);
                self.get_default_color()
            }
        }
    }
    // Default color for a file that doesn't have a valid configuration
    // rule.
    fn get_default_color(&self) -> String {
        "gray".to_string()
    }
}

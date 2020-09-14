use std::env;
use std::fmt::{self, Display};
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use std::string::ToString;

enum ConfigurationEntry {
    Pair(String, String),
    Flag(String),
}

impl Display for ConfigurationEntry {
    // Produces a string representation of ConfigurationEntry
    // that is suitable for storage in the configuration file.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigurationEntry::Flag(flag) => write!(f, "{}", flag),
            ConfigurationEntry::Pair(key, value) => write!(f, "{}={}", key, value),
        }
    }
}

pub struct Configuration {
    absolute_path: String,
    entries: Vec<ConfigurationEntry>,
}

impl Configuration {
    // This constructor pulls the configuration file into the program
    //
    // If the configuration file or directory do not exist, this constructor
    // will silent create the appropriate hierarchy with the default configuration.
    pub fn new() -> Configuration {
        let mut instance = Configuration {
            absolute_path: format!("{}/.pls/conf", env::var("HOME").unwrap()),
            entries: Vec::new(),
        };

        // Pull the configuration from the filesystem.
        instance.init();

        instance
    }
    // Probes the configuration directory and file before we can read
    // the data. If the configuration file does not exist, a default
    // configuration file is generated.
    fn init(&mut self) {
        // Check for pls conf dir
        let conf_dir = Path::new(&self.absolute_path).parent().unwrap();
        if !conf_dir.exists() {
            fs::create_dir(conf_dir).expect("Failed to initialize conf dir");
        }

        // Generate a default configuration file if it doesn't exist
        if !Path::new(&self.absolute_path).exists() {
            self.load_default_configuration()
                .expect("Unable to create conf file");
        }
    }
    // Loads the default values into the struct vector, and automatically
    // syncs the data into the configuration file for long-term storage.
    fn load_default_configuration(&mut self) -> io::Result<()> {
        self.entries.push(ConfigurationEntry::Pair(
            "*.js".to_string(),
            "yellow".to_string(),
        ));
        self.entries.push(ConfigurationEntry::Pair(
            "*.java".to_string(),
            "orange".to_string(),
        ));
        self.entries
            .push(ConfigurationEntry::Flag("no_permissions".to_string()));

        // Final step is to sync with the configuration file.
        self.sync()?;
        Ok(())
    }
    // Write the configuration in memory into the configuration file.
    fn sync(&self) -> io::Result<()> {
        let mut file = File::create(&self.absolute_path)?;

        for entry in &self.entries {
            file.write(entry.to_string().as_bytes())?;
            file.write("\n".as_bytes())?;
        }

        Ok(())
    }
}

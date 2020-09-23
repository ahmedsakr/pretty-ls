use regex::Regex;
use std::env;
use std::fmt::{self, Display};
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::ops::Drop;
use std::path::Path;
use std::string::ToString;

enum ConfigurationEntry {
    Pair(String, String),
    Flag(String),
}

impl Display for ConfigurationEntry {
    // Produces a string representation of ConfigurationEntry
    // that is suitable for storage in the configuration file.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfigurationEntry::Flag(flag) => write!(f, "{}", flag),
            ConfigurationEntry::Pair(key, value) => write!(f, "{}={}", key, value),
        }
    }
}

impl ConfigurationEntry {
    // Parses a single string to create a ConfigurationEntry
    // with respect to the structure of the string.
    fn new(value: &str) -> Self {
        let parts: Vec<&str> = value.split("=").collect();

        if parts.len() >= 2 {
            ConfigurationEntry::Pair(parts[0].to_string(), parts[1].to_string())
        } else {
            ConfigurationEntry::Flag(value.to_string())
        }
    }
    // Simple type check of this enum if it is ConfigurationEntry::Pair.
    fn is_pair(&self) -> bool {
        matches!(self, ConfigurationEntry::Pair(_, _))
    }
}

pub struct Configuration {
    absolute_path: Option<String>,
    dirty: bool,
    entries: Vec<ConfigurationEntry>,
}

impl Configuration {
    // This constructor pulls the configuration file into the program
    //
    // If the configuration file or directory do not exist, this constructor
    // will silent create the appropriate hierarchy with the default configuration.
    pub fn new() -> Self {
        let instance = Self {
            // The HOME environment variable must be present for us to sync/load a configuration
            // file.
            absolute_path: match env::var("HOME") {
                Ok(path) => Some(format!("{}/.pls/conf", path)),
                Err(_) => None,
            },
            dirty: false,
            entries: Vec::new(),
        };

        // Pull the configuration from the filesystem.
        instance.init()
    }
    // Add a configuration entry to this current instance
    pub fn add_entry(&mut self, entry: &str) {
        self.entries.push(ConfigurationEntry::new(entry));
    }
    // Attempts to get the associated value for the provided key.
    pub fn get_value(&self, key: &str) -> Result<Option<&str>, regex::Error> {
        for expr in self.entries.iter() {
            if let ConfigurationEntry::Pair(expr_key, value) = expr {
                if Regex::new(expr_key)?.is_match(key) {
                    return Ok(Some(value));
                }
            }
        }

        Ok(None)
    }
    // Probes the configuration directory and file before we can read
    // the data. If the configuration file does not exist, a default
    // configuration file is generated.
    fn init(mut self) -> Self {
        if let Some(config_path) = self.absolute_path.as_ref() {
            // Check for pls conf dir
            let conf_dir = Path::new(&config_path).parent().unwrap();
            if !conf_dir.exists() {
                fs::create_dir(conf_dir).expect("Failed to initialize conf dir");
            }

            // Try to load the configuration file into this instance
            if Path::new(&config_path).exists() {
                self.load().expect("Unable to read config file");
            }
        }

        // If the config file doesn't exitst or it is empty, we will use
        // default configuration.
        if self.entries.is_empty() {
            self.use_default_configuration()
        }

        self
    }
    // Loads the default values into the struct vector
    fn use_default_configuration(&mut self) {
        self.add_entry(".*.js$=#ffff00");
        self.add_entry(".*.java$=#efe232");
        self.add_entry("Cargo.toml$=#ffa500");
        self.add_entry("Cargo.lock$=#ffa500");
        self.add_entry(".*.rs=#ffa500");
        self.add_entry("no_permissions");

        // We need to write the default config when we exit
        self.dirty = true;
    }
    // Reads the entries from the configuration file into memory
    fn load(&mut self) -> io::Result<()> {
        let file = File::open(
            self.absolute_path
                .as_ref()
                .expect("load called with no config destination"),
        )?;
        let mut reader = BufReader::new(file);
        let mut line = String::new();
        // pattern to ignore section headers in config file
        let section_header = Regex::new(r".*\[.*\].*").unwrap();

        while reader.read_line(&mut line)? > 0 {
            // remove newline feed character
            line.pop();

            // Weed out unimportant data from the file
            if line != "" && !section_header.is_match(&line) {
                self.add_entry(&line);
            }

            line.clear();
        }

        Ok(())
    }
    // Write the configuration in memory into the configuration file.
    fn sync(&self) -> io::Result<()> {
        let mut file = File::create(
            self.absolute_path
                .as_ref()
                .expect("sync called with no config destination"),
        )?;

        // Section 1 is the colours for the provided regular expressions
        file.write("[colours]\n".as_bytes())?;
        for pair in self.entries.iter().filter(|entry| entry.is_pair()) {
            file.write(format!("{}\n", pair.to_string()).as_bytes())?;
        }

        // Section 2 is the flags for conditional behaviour
        file.write("\n[flags]\n".as_bytes())?;
        for flag in self.entries.iter().filter(|entry| !entry.is_pair()) {
            file.write(format!("{}\n", flag.to_string()).as_bytes())?;
        }

        file.flush()
    }
}

impl Drop for Configuration {
    // Before exiting, we need to persist the configuration settings if
    // we have made changes in memory.
    fn drop(&mut self) {
        if self.absolute_path.is_some() && self.dirty {
            self.sync().expect("Unable to sync configuration file");
        }
    }
}

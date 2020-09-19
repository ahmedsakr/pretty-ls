use regex::Regex;
use std::cmp::PartialEq;
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

impl PartialEq for ConfigurationEntry {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl ConfigurationEntry {
    // Parses a single string to create a ConfigurationEntry
    // with respect to the structure of the string.
    fn new(value: &str) -> Self {
        let expression = Regex::new(r".*=.*").unwrap();

        if expression.is_match(value) {
            let parts: Vec<&str> = value.split("=").collect();

            ConfigurationEntry::Pair(
                parts.get(0).unwrap().to_string(),
                parts.get(1).unwrap().to_string(),
            )
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
    absolute_path: String,
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
            absolute_path: format!("{}/.pls/conf", env::var("HOME").unwrap()),
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
    pub fn get_value(&self, key: &str) -> Result<Option<String>, regex::Error> {
        for expr in self.entries.iter() {
            match expr {
                ConfigurationEntry::Pair(expr_key, value) => {
                    if Regex::new(expr_key)?.is_match(key) {
                        return Ok(Some(value.to_string()));
                    }
                }
                _ => (),
            }
        }

        Ok(None)
    }
    // Probes the configuration directory and file before we can read
    // the data. If the configuration file does not exist, a default
    // configuration file is generated.
    fn init(mut self) -> Self {
        // Check for pls conf dir
        let conf_dir = Path::new(&self.absolute_path).parent().unwrap();
        if !conf_dir.exists() {
            fs::create_dir(conf_dir).expect("Failed to initialize conf dir");
        }

        // Try to load the configuration file into this instance, otherwise
        // defaulting to standard configuration.
        if Path::new(&self.absolute_path).exists() {
            self.load().expect("Unable to read config file");
        } else {
            self.use_default_configuration()
        }

        self
    }
    // Loads the default values into the struct vector
    fn use_default_configuration(&mut self) {
        self.add_entry(".*\\.js$=yellow");
        self.add_entry(".*\\.java$=orange");
        self.add_entry("no_permissions");

        // We need to write the default config when we exit
        self.dirty = true;
    }
    // Reads the entries from the configuration file into memory
    fn load(&mut self) -> io::Result<()> {
        let file = File::open(&self.absolute_path)?;
        let mut reader = BufReader::new(file);
        let mut line = String::new();
        // pattern to ignore section headers in config file
        let section_header = Regex::new(r".*\[.*\].*").unwrap();

        loop {
            match reader.read_line(&mut line)? {
                // EOF reached
                0 => break,
                _ => {
                    // remove newline feed character
                    line.pop();

                    // Weed out unimportant data from the file
                    if line != "" && !section_header.is_match(&line) {
                        self.add_entry(&line);
                    }

                    line.clear();
                }
            }
        }

        Ok(())
    }
    // Helper function for writing a whole line to a file
    fn file_writeln(&self, file: &mut File, data: &str) -> io::Result<()> {
        file.write(format!("{}\n", data).as_bytes())?;
        Ok(())
    }
    // Write the configuration in memory into the configuration file.
    fn sync(&self) -> io::Result<()> {
        let mut file = File::create(&self.absolute_path)?;

        // Section 1 is the colours for the provided regular expressions
        self.file_writeln(&mut file, "[colours]")?;
        for pair in self.entries.iter().filter(|entry| entry.is_pair()) {
            self.file_writeln(&mut file, &pair.to_string())?;
        }

        // Section 2 is the flags for conditional behaviour
        self.file_writeln(&mut file, "\n[flags]")?;
        for flag in self.entries.iter().filter(|entry| !entry.is_pair()) {
            self.file_writeln(&mut file, &flag.to_string())?;
        }

        file.flush()
    }
}

impl Drop for Configuration {
    // Before exiting, we need to persist the configuration settings if
    // we have made changes in memory.
    fn drop(&mut self) {
        if self.dirty {
            self.sync().expect("Unable to sync configuration file");
        }
    }
}

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

pub fn init() {
    let pls_conf = format!("{}/.pls/conf", env::var("HOME").unwrap());

    // Check for pls conf dir
    let path = Path::new(&pls_conf).parent().unwrap();
    if !path.exists() {
        fs::create_dir(path).expect("Failed to initialize conf dir");
    }

    // Check for pls conf file
    let path = Path::new(&pls_conf);
    if !path.exists() {
        write_default_configuration().expect("Unable to create conf file");
    }
}

fn get_default_configuration() -> Vec<ConfigurationEntry> {
    let mut config = Vec::new();
    config.push(ConfigurationEntry::Pair(
        "*.js".to_string(),
        "yellow".to_string(),
    ));
    config.push(ConfigurationEntry::Pair(
        "*.java".to_string(),
        "orange".to_string(),
    ));
    config.push(ConfigurationEntry::Flag("no_permissions".to_string()));

    config
}

fn write_default_configuration() -> io::Result<()> {
    let pls_conf = format!("{}/.pls/conf", env::var("HOME").unwrap());
    let mut file = File::create(&pls_conf)?;

    let config = get_default_configuration();
    for entry in &config {
        file.write(entry.to_string().as_bytes())?;
        file.write("\n".as_bytes())?;
    }

    Ok(())
}

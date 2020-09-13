use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

enum ConfigurationEntry {
    Pair(String, String),
    Flag(String),
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
        fs::File::create(path).expect("Unable to create conf file");
        write_default_configuration().expect("");
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
    let mut file = fs::File::create(&pls_conf)?;

    let config = get_default_configuration();
    for entry in &config {
        match entry {
            ConfigurationEntry::Pair(key, value) => {
                file.write(format!("{}={}", key, value).as_bytes())?;
            }
            ConfigurationEntry::Flag(flag) => {
                file.write(flag.as_bytes())?;
            }
        }

        file.write("\n".as_bytes())?;
    }

    Ok(())
}

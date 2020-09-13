use std::env;
use std::fs;
use std::io::ErrorKind;
use std::path::Path;

pub fn init() {
    let home = format!("{}/.pls", env::var("HOME").unwrap());
    let conf_dir = Path::new(&home);

    if !conf_dir.exists() {
        match fs::create_dir(conf_dir) {
            Err(err) => match err.kind() {
                ErrorKind::AlreadyExists => (),
                _ => panic!("Failed to initialize configuration: {}", err),
            },
            Ok(_) => (),
        }
    }
}

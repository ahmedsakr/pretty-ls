use std::io;

use crate::command::{self, SimpleCommand};

// Lists the content of the current working directory
pub fn list_dir(dir: Option<&String>) -> io::Result<Vec<String>> {
    let mut command = command::SystemCommand::new("ls");
    command.add_argument(match dir {
        Some(val) => Some(val.to_string()),
        None => Some(get_current_dir()?),
    });

    let out = command.run()?;
    let files = out.lines();

    // Must convert vector Lines to a vector of Strings
    let mut files_str: Vec<String> = Vec::new();
    files.for_each(|file| files_str.push(file.to_string()));

    Ok(files_str)
}

// Parses the absolute path of the current directory
fn get_current_dir() -> io::Result<String> {
    let out = command::SystemCommand::new("pwd").run()?;
    let current_dir = out.lines().next().expect("pwd failed");

    Ok(String::from(current_dir))
}

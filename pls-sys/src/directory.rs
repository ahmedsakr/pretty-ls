use std::io;

use crate::command::{self, SimpleCommand};

// Lists the content of the current working directory
pub fn list_dir(dir: Option<&String>) -> io::Result<Vec<String>> {
    let current_dir = get_current_dir()?;

    let command = command::SystemCommand {
        name: "ls",
        arguments: match dir {
            Some(x) => Some(x),
            // Use working directory if a path has not been provided.
            None => Some(&current_dir),
        },
    };

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

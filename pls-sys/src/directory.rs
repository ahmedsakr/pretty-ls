use std::io::{Error as IOError, ErrorKind};
use std::str;

use crate::command::{self, SystemCommand};

// Lists the content of the current working directory
pub fn list_dir(dir: Option<&String>) -> Result<Vec<String>, IOError> {
    let current_dir = get_current_dir()?;

    let command = command::Command {
        name: "ls",
        arguments: match dir {
            Some(x) => Some(x),
            // Use working directory if a path has not been provided.
            None => Some(&current_dir),
        },
    };

    let out = command.run()?;

    if let Some(0) = out.status.code() {
        let files = str::from_utf8(&out.stdout)
            .expect("Failed to decode")
            .lines();

        // Must convert vector Lines to a vector of Strings
        let mut files_str: Vec<String> = Vec::new();
        files.for_each(|file| files_str.push(file.to_string()));

        return Ok(files_str);
    }

    Err(IOError::new(ErrorKind::InvalidData, "ls failed"))
}

// Parses the absolute path of the current directory
fn get_current_dir() -> Result<String, IOError> {
    let out = command::Command::new("pwd").run()?;
    let dir = str::from_utf8(&out.stdout).expect("Failed to decode");

    let new_line_index = dir
        .chars()
        // Snip the string at the end of the line to retain the path only
        .position(|c| c == '\n')
        .expect("Bad output from ls");

    Ok(dir[..new_line_index].to_string())
}

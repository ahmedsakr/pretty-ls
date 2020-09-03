use std::io::{Error as IOError, ErrorKind};
use std::process::Command;
use std::str;

// Lists the content of the current working directory
pub fn list_dir(dir: Option<&String>) -> Result<Vec<String>, IOError> {
    let mut command = Command::new("ls");

    let out = match dir {
        Some(directory) => command.arg(directory).output()?,

        // Use the current working directory by default
        None => command.arg(get_current_dir()?.to_string()).output()?,
    };

    println!("{}", out.status);

    if let Some(0) = out.status.code() {

        let files = str::from_utf8(&out.stdout)
        .expect("Failed to decode")
        .lines();

        // Must convert vector Lines to a vector of Strings
        let mut files_str: Vec<String> = Vec::new();
        files.for_each(|file| files_str.push(file.to_string()));

        Ok(files_str)
    } else {
        Err(IOError::new(ErrorKind::InvalidData, "ls failed"))
    }
}

// Parses the absolute path of the current directory
fn get_current_dir() -> Result<String, IOError> {
    let out = Command::new("pwd").output()?;
    let dir = str::from_utf8(&out.stdout).expect("Failed to decode");

    let new_line_index = dir
        .chars()
        // Snip the string at the end of the line to retain the path only
        .position(|c| c == '\n')
        .expect("Bad output from ls");

    Ok(dir[..new_line_index].to_string())
}

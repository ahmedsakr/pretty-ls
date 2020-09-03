use std::io;
use std::process::Command;
use std::str;

// Lists the content of the current working directory
pub fn list_dir(dir: Option<&String>) -> Result<Vec<String>, io::Error> {
    let out = match dir {
        Some(directory) => Command::new("ls").arg(directory).output()?,

        // Use the current working directory by default
        None => Command::new("ls")
            .arg(get_current_dir()?.to_string())
            .output()?,
    };

    let files = str::from_utf8(&out.stdout)
        .expect("Failed to decode")
        .lines();

    // Must convert vector Lines to a vector of Strings
    let mut files_str: Vec<String> = Vec::new();
    files.for_each(|file| files_str.push(file.to_string()));

    Ok(files_str)
}

// Parses the absolute path of the current directory
fn get_current_dir() -> Result<String, io::Error> {
    let out = Command::new("pwd").output()?;
    let dir = str::from_utf8(&out.stdout).expect("Failed to decode");

    let new_line_index = dir
        .chars()
        // Snip the string at the end of the line to retain the path only
        .position(|c| c == '\n')
        .expect("Bad output from ls");

    Ok(dir[..new_line_index].to_string())
}

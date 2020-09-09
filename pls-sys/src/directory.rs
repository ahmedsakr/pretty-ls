use std::io;

use crate::command::{self, SimpleCommand};

// Lists the content of the current working directory
pub fn list_dir(dir: Option<&String>) -> io::Result<Vec<String>> {
    let mut command = command::SystemCommand::new("ls");
    command.add_argument(match dir {
        Some(val) => Some(val.to_string()),
        None => Some(get_current_dir()?),
    });

    Ok(command.run()?)
}

// Parses the absolute path of the current directory
fn get_current_dir() -> io::Result<String> {
    let out = command::SystemCommand::new("pwd").run()?;
    Ok(String::from(out.first().expect("pwd returned nothing")))
}

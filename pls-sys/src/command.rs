use std::io::{self, Error, ErrorKind};
use std::process;

pub trait SimpleCommand {
    // Executes the command, returning the standad output for the
    // command.
    fn run(&self) -> io::Result<String>;
}

pub struct SystemCommand<'a> {
    // The executable name
    pub name: &'a str,
    // Everything else specified after executable name
    pub arguments: Option<&'a str>,
}

impl<'a> SystemCommand<'a> {
    // Associated function to initialize a command with
    // no arguments.
    pub fn new(name: &'a str) -> SystemCommand {
        SystemCommand {
            name,
            arguments: None,
        }
    }
}

impl<'a> SimpleCommand for SystemCommand<'a> {
    // Runs the command by constructing the qualified command
    // from the command name and arguments.
    fn run(&self) -> io::Result<String> {
        let mut command = process::Command::new(&self.name);

        if let Some(args) = &self.arguments {
            let args_vector: Vec<&str> = args.split(" ").collect();
            for arg in &args_vector {
                command.arg(arg);
            }
        }

        // Grab the command output
        let out = command
            .output()
            .expect(&format!("Unable to execute {cmd}!", cmd = self.name));

        match out.status.code() {
            Some(0) => Ok(String::from_utf8(out.stdout).expect("Failed to decode utf-8")),

            // Everything that isn't a 0 return value is considered as a failure.
            _ => Err(Error::new(
                ErrorKind::InvalidData,
                format!("{cmd} failed", cmd = self.name),
            )),
        }
    }
}

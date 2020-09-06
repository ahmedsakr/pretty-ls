use std::io::{self, Error, ErrorKind};
use std::process;

pub trait SystemCommand {
    // Executes the command, returning the io::Result for the
    // system command.
    fn run(&self) -> io::Result<process::Output>;
}

pub struct Command<'a> {
    // The executable name
    pub name: &'a str,
    // Everything else specified after executable name
    pub arguments: Option<&'a str>,
}

impl<'a> Command<'a> {
    // Associated function to initialize a command with
    // no arguments.
    pub fn new(name: &'a str) -> Command {
        Command {
            name,
            arguments: None,
        }
    }
}

impl<'a> SystemCommand for Command<'a> {
    // Runs the command by constructing the qualified command
    // from the command name and arguments.
    fn run(&self) -> io::Result<process::Output> {
        let mut command = process::Command::new(&self.name);

        if let Some(args) = &self.arguments {
            let args_vector: Vec<&str> = args.split(" ").collect();
            for arg in &args_vector {
                command.arg(arg);
            }
        }

        // Execute and return the command output
        let out = command.output().expect(&format!("Unable to execute {cmd}!", cmd = self.name));
        match out.status.code() {
            Some(0) => Ok(out),
            _ => Err(Error::new(ErrorKind::InvalidData, format!("{cmd} failed", cmd = self.name)))
        }
    }
}

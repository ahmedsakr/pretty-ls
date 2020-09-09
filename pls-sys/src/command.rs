use std::io::{self, Error, ErrorKind};
use std::process;

pub trait SimpleCommand {
    // Adds an argument to the chain of arguments for this command.
    // If the argument is None, nothing should be done.
    fn add_argument(&mut self, arg: Option<String>);
    // Executes the command, returning the standad output for the
    // command.
    fn run(&self) -> io::Result<Vec<String>>;
}

pub struct SystemCommand<'a> {
    // The executable name
    name: &'a str,
    // Everything else specified after executable name
    arguments: Vec<String>,
}

impl<'a> SystemCommand<'a> {
    // Associated function to initialize a command with
    // no arguments.
    pub fn new(name: &'a str) -> SystemCommand {
        SystemCommand {
            name,
            arguments: Vec::new(),
        }
    }
    // Transforms raw stdout byte vector into a vector of Strings.
    fn parse_stdout(&self, raw_bytes: Vec<u8>) -> Vec<String> {
        let mut stdout: Vec<String> = Vec::new();
        let raw_output = String::from_utf8(raw_bytes).expect("Failed to decode utf-8");

        raw_output
            .lines()
            .for_each(|line| stdout.push(line.to_string()));

        stdout
    }
}

impl<'a> SimpleCommand for SystemCommand<'a> {
    // Runs the command by constructing the qualified command
    // from the command name and arguments.
    fn run(&self) -> io::Result<Vec<String>> {
        let mut command = process::Command::new(&self.name);

        if !self.arguments.is_empty() {
            for arg in &self.arguments {
                command.arg(arg);
            }
        }

        // Grab the command output
        let out = command
            .output()
            .expect(&format!("Unable to execute {cmd}!", cmd = self.name));

        match out.status.code() {
            Some(0) => Ok(self.parse_stdout(out.stdout)),

            // Everything that isn't a 0 return value is considered as a failure.
            _ => Err(Error::new(
                ErrorKind::InvalidData,
                format!("{cmd} failed", cmd = self.name),
            )),
        }
    }
    // Appends a string argument to the command chain.
    // If the argument is None, Nothing is done.
    fn add_argument(&mut self, arg: Option<String>) {
        if arg.is_some() {
            self.arguments.push(arg.unwrap());
        }
    }
}

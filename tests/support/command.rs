use std::fmt::{Debug, Error, Formatter};
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::str;

pub struct TestCommand {
    cwd: PathBuf,
    args: Vec<String>,
    env_vars: Vec<(String, String)>,
}

/// Run diesel-cli commands programmatically
#[allow(dead_code)]
impl TestCommand {
    /// Build a new OS command
    pub fn new(cwd: &Path, subcommand: &str) -> Self {
        TestCommand {
            cwd: cwd.into(),
            args: vec![subcommand.into()],
            env_vars: Vec::new(),
        }
    }

    /// Add an argument to the command
    pub fn arg<S: Into<String>>(mut self, value: S) -> Self {
        self.args.push(value.into());
        self
    }

    pub fn args<I>(self, values: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<String>,
    {
        values.into_iter().fold(self, |c, value| c.arg(value))
    }

    /// Add an environment variable to the command
    pub fn env(mut self, key: &str, value: &str) -> Self {
        self.env_vars.push((key.into(), value.into()));
        self
    }

    /// Run the command and store the output
    pub fn run(self) -> CommandResult {
        let output = self.build_command().output().unwrap();
        CommandResult { output: output }
    }

    fn build_command(&self) -> Command {
        let mut command = Command::new("diesel");
        command.args(&self.args).current_dir(&self.cwd);
        for &(ref k, ref v) in self.env_vars.iter() {
            command.env(&k, &v);
        }
        command
    }
}

pub struct CommandResult {
    output: Output,
}

#[allow(dead_code)]
impl CommandResult {
    pub fn is_success(&self) -> bool {
        self.output.status.success()
    }

    pub fn stdout(&self) -> &str {
        str::from_utf8(&self.output.stdout).unwrap()
    }

    pub fn stderr(&self) -> &str {
        str::from_utf8(&self.output.stderr).unwrap()
    }

    pub fn code(&self) -> i32 {
        self.output.status.code().unwrap()
    }

    pub fn result(self) -> Result<Self, Self> {
        if self.is_success() {
            Ok(self)
        } else {
            Err(self)
        }
    }
}

impl Debug for CommandResult {
    fn fmt(&self, out: &mut Formatter) -> Result<(), Error> {
        write!(out, "stdout: {}\nstderr: {}", self.stdout(), self.stderr())
    }
}

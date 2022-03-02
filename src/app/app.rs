use crate::app::Action;
use crate::app::Argument;
use crate::app::cmd::Command;
use std::default::Default;
use std::env;

pub struct Application {
    name: String,
    usage: Option<String>,
    author: Option<String>,
    before: Option<Action>,
    after: Option<Action>,
    action: Option<Action>,
    version: Option<String>,
    help_name: Option<String>,
    usage_text: Option<String>,
    description: Option<String>,
    command_vec: Vec<Command>,
    argument_vec: Vec<Argument>,
}

impl Application {
    pub fn new(name: String) -> Self {
        Application {
            name,
            ..Default::default()
        }
    }

    pub fn arg(mut self, arg: Argument) -> Self {
        self.argument_vec.push(arg);
        self
    }

    pub fn args(mut self, args: Vec<Argument>) -> Self {
        for arg in args {
            self.argument_vec.push(arg);
        }
        self
    }

    pub fn usage(mut self, usage: &str) -> Self {
        self.usage = Some(usage.to_owned());
        self
    }

    pub fn subcommand(mut self, command: Command) -> Self {
        self.command_vec.push(command);
        self
    }

    pub fn subcommands(mut self, commands: Vec<Command>) -> Self {
        for cmd in commands {
            self.command_vec.push(cmd);
        }
        self
    }

    pub fn run_args(&mut self, args: Vec<String>) -> anyhow::Result<()>{
        Ok(())
    }

    pub fn run(&mut self) {
        let args: Vec<String> = env::args().collect();
        let r = self.run_args(args);
        if let Err(e) = r {
            println!("Error {}", e)
        }
    }
}
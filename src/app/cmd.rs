use crate::app::Action;
use crate::app::Argument;

pub struct Command {
    name: String,
    usage: Option<String>,
    before: Option<Action>,
    after: Option<Action>,
    action: Option<Action>,
    aliases: Vec<String>,
    help_name: Option<String>,
    usage_text: Option<String>,
    version: Option<String>,
    description: Option<String>,
    sub_command: Vec<Command>,
    argument_vec: Vec<Argument>,
    hide_help: bool,
    hide_help_command: bool,
    hidden: bool,
}


impl Command {
    pub fn new(name: String) -> Self {
        Command {
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

    pub fn run(&mut self, args: Vec<String>) -> anyhow::Result<()> {
        Ok(())
    }
}
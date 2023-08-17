use std::vec;
use hashbrown::HashMap;

use crate::cli::ArgParse;

pub mod install;

#[derive(Clone)]
pub struct Command {
    name: String,
    description: String,
    aliases: Vec<String>,
    run: fn(ArgParse),
}
impl Command {
    pub fn new(name: String, description: String, aliases: Vec<String>, run: fn(ArgParse)) -> Command {
        Command {
            name,
            description,
            aliases,
            run,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn get_aliases(&self) -> &Vec<String> {
        &self.aliases
    }

    pub fn get_run(&self) -> &fn(ArgParse) {
        &self.run
    }
}

pub struct CommandHolder {
    commands: Vec<Command>,
    command_map: HashMap<String, Command>,
}
impl CommandHolder {
    pub fn new(commands: Vec<Command>) -> CommandHolder {
        let mut command_map: HashMap<String, Command> = HashMap::new();

        for cmd in &commands {
            let cmd_clone = cmd.clone();

            command_map.insert(cmd.get_name().to_string(), cmd.clone());

            for alias in cmd.get_aliases() {
                command_map.insert(alias.to_string(), cmd_clone.clone());
            }
        }

        CommandHolder {
            commands,
            command_map,
        }
    }

    pub fn get_commands(&self) -> &Vec<Command> {
        &self.commands
    }
    
    pub fn is_valid(&self, command: &str) -> Option<Command> {
        self.command_map
            .get(command)
            .cloned()
    }
}

pub fn init() -> CommandHolder {
    let mut commands: Vec<Command> = Vec::new();
    
    commands.push(Command::new(
        String::from("install"),
        String::from("Install a package"),
        vec![String::from("i")],
        install::run,
    ));

    CommandHolder::new(commands)
}
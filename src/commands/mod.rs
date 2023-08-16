use std::vec;

use crate::cli::ArgParse;

pub mod install;

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

pub fn init() -> Vec<Command> {
    let mut commands: Vec<Command> = Vec::new();
    
    commands.push(Command::new(
        String::from("install"),
        String::from("Install a package"),
        vec![String::from("i")],
        install::run,
    ));

    commands
}
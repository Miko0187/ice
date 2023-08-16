use crate::{
    // Macro imports
    throw, 
    error,
    _color_output
};

#[derive(Debug)]
pub struct ArgParse {
    command: String,
    args: Vec<String>,
}

impl ArgParse {
    pub fn new() -> ArgParse {
        let args: Vec<String> = std::env::args().collect();

        let command = match args.get(1) {
            Some(command) => command.to_string(),
            None => {
                throw!("No command provided");
            }
        };

        let args = match args.get(2..) {
            Some(args) => args.to_vec(),
            None => {
                vec![]
            }
        };

        ArgParse { command, args }
    }

    pub fn get_command(&self) -> &String {
        &self.command
    }

    pub fn get_args(&self) -> &Vec<String> {
        &self.args
    }

    pub fn get_arg(&self, index: usize) -> Option<String> {
        let arg = self.args.get(index);
        if arg.is_none() {
            return None;
        }
        Some(arg.unwrap().to_string())
    }
}

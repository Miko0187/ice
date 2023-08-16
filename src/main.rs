use crate::commands::Command;

mod cli;
pub mod utils;
pub mod commands;


fn main() {
    let commands: Vec<Command> = commands::init();
    let args = cli::ArgParse::new();

    match args.get_command().as_str() {
        "install" | "i" => commands::install::run(args),
        _ => {
            throw!("Unknown command: {}", args.get_command());
        },
    }
}

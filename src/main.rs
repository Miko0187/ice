mod cli;
pub mod utils;
pub mod commands;


fn main() {
    let commands = commands::init();
    let args = cli::ArgParse::new();

    match commands.is_valid(args.get_command()) {
        Some(cmd) => {
            (cmd.get_run())(args);
        }
        None => {
            throw!("Command not found: {}", args.get_command());
        }
    }
}

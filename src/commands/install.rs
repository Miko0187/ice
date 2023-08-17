use crate::{
    // Macro imports
    throw,
    error,
    info,
    _color_output,
    // Module imports
    cli::ArgParse, 
};

pub fn run(args: ArgParse) {
    let package = args.get_arg(0);
    if package.is_none() {
        throw!("No package provided");
    }
    info!("Installing package: {}", package.unwrap());

    
}
use minreq;

use crate::{
    throw,
    error,
    info,
    _color_output,
    cli::ArgParse, 
};
use super::{
    Package,
    NPM_REGISTRY,
};


fn get_package(package: &String, version: &String) -> Result<Package, minreq::Error> {
    let response = minreq::get(format!("{}{}/{}", NPM_REGISTRY, package, version)).send()?;

    match response.status_code {
        200 => {
            response.json::<Package>()
        }
        404 => {
            throw!("Package/version not found");
        }
        _ => {
            throw!("Error getting package: {}", response.status_code);
        }
    }
}

pub fn run(args: ArgParse) {
    let package = args.get_arg(0);
    if package.is_none() {
        throw!("No package provided");
    }

    let mut version = args.get_arg(1);
    if version.is_none() {
        version = Some("latest".to_string())
    }

    let unwrapped_package = package.unwrap();

    info!("Installing package: {}", unwrapped_package);

    let package = get_package(&unwrapped_package, &version.unwrap());
    if package.is_err() {
        throw!("Error getting package: {}", package.err().unwrap());
    }
    
    println!("{:?}", package.unwrap());
}
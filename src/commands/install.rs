use minreq;
use minreq::Error as MinreqError;
use std::{
    env::current_dir, 
    io::Cursor,
    fs, collections::HashSet,
};
use flate2::read::GzDecoder;
use tar::Archive;
use regex::Regex;
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
    let mut parsed = version.replace("~", "").replace("^", "");
    let re = Regex::new(r#"\d+\.\d+\.\d+"#).unwrap();

    if let Some(caps) = re.captures(&parsed) {
        parsed = caps.get(0).unwrap().as_str().to_string();
    }


    let response = minreq::get(format!("{}{}/{}", NPM_REGISTRY, package, parsed)).send()?;

    match response.status_code {
        200 => {
            response.json::<Package>()
        }
        404 => {
            throw!("Package/version not found");
        }
        _ => {
            throw!("Failed to fetch package: {}", response.status_code);
        }
    }
}

fn download_package(package: &Package) -> Result<(), minreq::Error> {
    let response = minreq::get(&package.distribution.tarball_url).send()?;

    match response.status_code {
        200 => {
            let gz_decoder = GzDecoder::new(Cursor::new(response.as_bytes()));
            let mut archive = Archive::new(gz_decoder);

            let node_modules_path = current_dir()
                .expect("Failed to get current directory")
                .join("node_modules");

            let package_path = node_modules_path.join(&package.name);

            if !package_path.exists() {
                fs::create_dir(&package_path).expect("Failed to create package folder");
            }

            archive.unpack(&package_path)?;

            Ok(())
        }
        _ => {
            throw!("Error downloading package: {}", response.status_code);
        }
    }
}

fn download_and_resolve_dependencies(
    package: &Package,
    visited_packages: &mut HashSet<String>,
) -> Result<(), MinreqError> {
    if visited_packages.contains(&package.name) {
        return Ok(());
    }

    visited_packages.insert(package.name.clone());

    info!("Downloading package: {}", package.name);

    download_package(package)?;

    if let Some(dependencies) = &package.dependencies {
        for (dependency_name, dependency_version) in dependencies {
            info!("Resolving dependency: {}", dependency_name);
            info!("npm reg url: {}{}/{}", NPM_REGISTRY, dependency_name, dependency_version);

            let dependency_package = get_package(dependency_name, dependency_version)?;

            info!("Found dependency: {}", dependency_package.name);

            download_and_resolve_dependencies(&dependency_package, visited_packages)?;
        }
    }

    Ok(())
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

    let now = std::time::Instant::now();

    info!("Installing package: {}", unwrapped_package);

    let cwd = current_dir().unwrap();
    let node_modules = cwd.join("node_modules");
    if !node_modules.exists() || !node_modules.is_dir() {
        fs::create_dir(node_modules).unwrap();
    }

    let package = get_package(&unwrapped_package, &version.unwrap());
    if package.is_err() {
        throw!("Failed to get package: {}", package.err().unwrap());
    }

    info!("Found version: {}", package.as_ref().unwrap().version);
    
    if let Err(err) = download_and_resolve_dependencies(&package.unwrap(), &mut HashSet::new()) {
        throw!("Failed to download package: {}", err);
    }

    info!("Done in {}s", now.elapsed().as_secs());
}
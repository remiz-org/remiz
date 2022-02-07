#[macro_use]
extern crate log;

mod args;
mod build_package;
mod deploy_package;
mod diff;
mod errors;
mod global_configuration;
pub mod helpers;
mod inspect;
mod logger;
mod metadata;
mod package;
mod package_configuration;
mod subpackage;
mod unpack;
mod store;

use std::process;

use args::{parse_args, Command};
use build_package::build;
use deploy_package::deploy;
use diff::diff;
use inspect::inspect;
use unpack::unpack;


fn main() {
    let args = parse_args();
    logger::init(args.verbose);

    match args.cmd {
        Command::Build {
            path_to_config_file,
            global_config_file,
        } => {
            match build(path_to_config_file, global_config_file) {
                Ok(()) => info!("Successfully build package."),
                Err(err) => {
                    error!("Failed to build package ({}).", err);
                    process::exit(1);
                }
            };
        }
        Command::Deploy {
            path_to_package_file,
            env,
            global_config_file,
        } => match deploy(path_to_package_file, env, global_config_file) {
            Ok(()) => info!("Successfully deployed package."),
            Err(err) => error!("Failed to deploy package ({}).", err),
        },
        Command::Inspect {
            path_to_package_file,
        } => {
            inspect(path_to_package_file);
        }
        Command::Unpack {
            path_to_package_file,
            destination_path,
        } => match unpack(path_to_package_file, destination_path) {
            Ok(()) => info!("Successfully unpacked package."),
            Err(err) => error!("Failed to unpack package ({}).", err),
        },
        Command::Diff {
            path_to_package_1,
            path_to_package_2,
        } => {
            diff(path_to_package_1, path_to_package_2);
        }
    }
}

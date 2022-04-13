use clap::Parser;
use std::path::PathBuf;

/// Packaging tool to build and deploy packages
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub cmd: Command,
    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    pub verbose: u8,
}

#[derive(Parser, Debug)]
pub enum Command {
    /// Build a package file from a configuration file.
    Build {
        /// Path to the configuration file (in TOML).
        #[structopt(parse(from_os_str))]
        path_to_config_file: PathBuf,
        /// Overide the path of the global configuration file (optional)
        #[structopt(short, long, parse(from_os_str))]
        global_config_file: Option<PathBuf>,
        /// Extra arguments (after '--') can be passed to package builders
        #[clap(multiple_occurrences = true, last = true)]
        extra_args: Vec<String>,
    },
    /// Deploy a project from a package file
    Deploy {
        /// Path to the package file (.pack).
        #[structopt(parse(from_os_str))]
        path_to_package_file: PathBuf,
        /// Environment to deploy to.
        #[clap(short, long)]
        env: String,
        /// Overide the path of the global configuration file (optional)
        #[structopt(short, long, parse(from_os_str))]
        global_config_file: Option<PathBuf>,
        /// Extra arguments (after '--') can be passed to package deployers
        #[clap(multiple_occurrences = true, last = true)]
        extra_args: Vec<String>,
    },
    /// Inspect a package file
    Inspect {
        /// Path to the package file (.pack).
        #[structopt(parse(from_os_str))]
        path_to_package_file: PathBuf,
    },
    /// Unpack a package file into a folder (to inspect it for example).
    Unpack {
        /// Path to the package file (.pack).
        #[structopt(parse(from_os_str))]
        path_to_package_file: PathBuf,
        /// Destination path (optional, if not specified, package path will be used)
        #[structopt(parse(from_os_str))]
        destination_path: Option<PathBuf>,
    },
    /// Compare the content of two package files.
    /// The output will be a list of metadata & subpackages that are different.
    /// More info are displayed if subpackager implements the `info` argument.
    Diff {
        /// Path to the first package file (.pack).
        #[structopt(parse(from_os_str))]
        path_to_package_1: PathBuf,
        /// Path to the second package file (.pack).
        #[structopt(parse(from_os_str))]
        path_to_package_2: PathBuf,
    },
}

pub fn parse_args() -> Args {
    trace!("Parsing arguments...");
    Args::parse()
}

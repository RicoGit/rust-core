mod import_manager;
mod rust_file_walker;

use eyre::Result;
use serde_derive::Deserialize;
use std::collections::{HashMap, VecDeque};
use std::{
    fs::File,
    io::{self, Read},
    path::Path,
};

use crate::import_manager::ImportProcessor;
use crate::rust_file_walker::FileProcessor;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    /// Input folder for grouping
    #[structopt(short, long, default_value = ".", parse(from_os_str))]
    input: PathBuf,

    /// Config file with defined rules of grouping imports
    #[structopt(short, long, default_value = ".fmt_import.toml", parse(from_os_str))]
    config: PathBuf,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    /// Ignored folders and files
    ignore: Vec<String>,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let config = read_config(&opt)?;

    let import_processor = ImportProcessor {};
    let file_processor = FileProcessor::new(opt.input, config, import_processor)?;

    let affected = file_processor.start()?;
    println!("Affected files: {}", affected);

    Ok(())
}

fn read_config(opt: &Opt) -> Result<Config> {
    let path = Path::new(&opt.config);
    println!("config file: {:?}", path);
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let config = toml::from_str(&content)?;
    Ok(config)
}

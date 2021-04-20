use std::path::PathBuf;
use structopt::StructOpt;
use serde::{Deserialize, Serialize};

#[derive(StructOpt, Debug)]
#[structopt(name = "Find_Biggest_File")]
struct CliArgs {

    #[structopt(short, long, parse(from_os_str))]
    config_file_path: Option<PathBuf>,

    #[structopt(short,long)]
    init: bool
}

#[derive(Debug, Serialize, Deserialize)]
struct BiggestFileInFolderContainer {
    folder: String,
    file_type: String
}
#[derive(Debug, Serialize, Deserialize)]
enum SearchPatternType {
    Basic(String),
    BiggestFileInFolder(BiggestFileInFolderContainer)
}
#[derive(Debug, Serialize, Deserialize)]
struct Config {
    search_patterns: Vec<SearchPatternType>,
    root_folder: PathBuf
}

#[derive(Debug, thiserror::Error)]
enum ProgError {
    #[error("i/o error: {0}")]
    Io(#[from] std::io::Error),
    #[error("serde error: {0}")]
    Serde(#[from] serde_json::Error)
}

fn create_default_config() -> Result<(),ProgError> {
    let example_file_container = BiggestFileInFolderContainer {
        folder: String::from("Example"),
        file_type: String::from("txt")
    };

    let default_config = Config {
        search_patterns: vec![SearchPatternType::Basic(String::from("example.txt")), SearchPatternType::BiggestFileInFolder(example_file_container)],
        root_folder: PathBuf::from(".")
    };

    let json_text = serde_json::to_string_pretty(&default_config)?;

    std::fs::write("Example_Config.json", json_text)?;

    Ok(())
}

fn parse_config(config_path:PathBuf) -> Result<Config,ProgError> {
    let file_contents = std::fs::read_to_string(config_path)?;
    let config = serde_json::from_str(&file_contents)?;
    Ok(config)
}

fn search_for_biggest_files(config:Config) -> Result<(),ProgError> {
    Ok(())
}

fn main() {
    let args = CliArgs::from_args();
    println!("{:#?}", args);

    if args.init {
        match create_default_config() {
            Ok(x) => x,
            Err(e) => panic!("{:#?}", e)
        }
    }

    if let Some(file_path) = args.config_file_path {
        let configRes = parse_config(file_path);
        match configRes {
            Ok(config) => println!("{:#?}", config),
            Err(e) =>panic!("{:#?}", e)
        };
    }
}

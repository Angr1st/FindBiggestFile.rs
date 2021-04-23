use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Find_Biggest_File")]
struct CliArgs {
    #[structopt(short, long, parse(from_os_str))]
    config_file_path: Option<PathBuf>,

    #[structopt(short, long)]
    init: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
struct BiggestFileInFolderContainer {
    folder: String,
    file_type: String,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
enum SearchPatternType {
    Basic(String),
    BiggestFileInFolder(BiggestFileInFolderContainer),
}
#[derive(Debug, Serialize, Deserialize)]
struct Config {
    search_patterns: Vec<SearchPatternType>,
    root_folder: PathBuf,
}

#[derive(Debug, thiserror::Error)]
enum ProgError {
    #[error("i/o error: {0}")]
    Io(#[from] std::io::Error),
    #[error("serde error: {0}")]
    Serde(#[from] serde_json::Error),
}

fn create_default_config() -> Result<(), ProgError> {
    let example_file_container = BiggestFileInFolderContainer {
        folder: String::from("Example"),
        file_type: String::from("txt"),
    };

    let default_config = Config {
        search_patterns: vec![
            SearchPatternType::Basic(String::from("example.txt")),
            SearchPatternType::BiggestFileInFolder(example_file_container),
        ],
        root_folder: PathBuf::from("."),
    };

    let json_text = serde_json::to_string_pretty(&default_config)?;

    std::fs::write("Example_Config.json", json_text)?;

    Ok(())
}

fn parse_config(config_path: PathBuf) -> Result<Config, ProgError> {
    let file_contents = std::fs::read_to_string(config_path)?;
    let config = serde_json::from_str(&file_contents)?;
    Ok(config)
}

use walkdir::WalkDir;

struct FileFoundResult {
    file: PathBuf,
    size: u64,
}
struct NoFileFoundResult {
    message: String,
}

impl NoFileFoundResult {
    fn new(search_pattern: &SearchPatternType) -> NoFileFoundResult {
        match search_pattern {
            SearchPatternType::Basic(file_name) => NoFileFoundResult {
                message: String::from(format!("No file found with the name: {}!", file_name)),
            },
            SearchPatternType::BiggestFileInFolder(container) => NoFileFoundResult {
                message: String::from(format!(
                    "Inside the folder: {}, no file was found with extension: {}!",
                    container.folder, container.file_type
                )),
            },
        }
    }
}

enum SearchResult {
    FileFound(FileFoundResult),
    NoFileFound(NoFileFoundResult),
}

fn create_initial_search_result(mut config: Config) -> (Vec<SearchResult>,Vec<SearchPatternType>, PathBuf) {
    let mut result = Vec::new();
    config.search_patterns.dedup();
    for search_pattern in config.search_patterns.clone().into_iter() {
        let file_search_container = SearchResult::NoFileFound(NoFileFoundResult::new(&search_pattern));
        result.push(file_search_container);
    }

    (result, config.search_patterns, config.root_folder)
}

fn search_for_biggest_files(config: Config) -> Result<(), ProgError> {
    let (search_results, search_patterns, root_folder) = create_initial_search_result(config);
    let mut results: HashMap<SearchPatternType,SearchResult> = HashMap::new();
    for (search_pattern,search_result) in search_patterns.clone().into_iter().zip(search_results) {
        results.insert(search_pattern, search_result);
    }
    for entry in WalkDir::new(root_folder).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            let file_name = path.file_name().unwrap();
            let extension = path.extension();
            let directory = path.parent().and_then(|f| f.components().last());
            let file_size = path.metadata()?.len();
            for search_pattern in &search_patterns {
                match search_pattern {
                    SearchPatternType::Basic(searched_file) => {
                        if std::ffi::OsString::from(&searched_file) == file_name {
                            let file_found = match results.get(search_pattern).unwrap() {
                                SearchResult::NoFileFound(_) => {
                                    Some(SearchResult::FileFound(FileFoundResult {
                                        size: file_size,
                                        file: path.to_path_buf(),
                                    }))
                                }
                                SearchResult::FileFound(file_found) => {
                                    if file_found.size < file_size {
                                        Some(SearchResult::FileFound(FileFoundResult {
                                            size: file_size,
                                            file: path.to_path_buf(),
                                        }))
                                    } else {
                                        None
                                    }
                                }
                            };
                            if file_found.is_some() {
                                results.insert(search_pattern.clone(),file_found.unwrap());
                            }
                        }
                    }
                    SearchPatternType::BiggestFileInFolder(biggest_file_in_folder_container) => {
                        if directory.is_some()
                            && directory.unwrap().as_os_str()
                                == std::ffi::OsString::from(&biggest_file_in_folder_container.folder)
                            && extension.is_some()
                            && extension.unwrap()
                                == std::ffi::OsString::from(
                                    &biggest_file_in_folder_container.file_type,
                                )
                        {
                            let file_found = match results.get(search_pattern).unwrap() {
                                SearchResult::NoFileFound(_) => {
                                    Some(SearchResult::FileFound(FileFoundResult {
                                        size: file_size,
                                        file: path.to_path_buf(),
                                    }))
                                }
                                SearchResult::FileFound(file_found) => {
                                    if file_found.size < file_size {
                                        Some(SearchResult::FileFound(FileFoundResult {
                                            size: file_size,
                                            file: path.to_path_buf(),
                                        }))
                                    } else {
                                        None
                                    }
                                }
                            };
                            if file_found.is_some() {
                                results.insert(search_pattern.clone(),file_found.unwrap());
                            }
                        }
                    }
                }
            }
        }
    }

    for result in results.values() {
        match result {
            SearchResult::NoFileFound(msg) => println!("{}", msg.message),
            SearchResult::FileFound(file_found) => println!("File: {}; Size: {} bytes", file_found.file.display(), file_found.size)
        }       
    }

    Ok(())
}

fn main() {
    let args = CliArgs::from_args();

    if args.init {
        match create_default_config() {
            Ok(x) => x,
            Err(e) => panic!("{:#?}", e),
        }
    }

    if let Some(file_path) = args.config_file_path {
        let config_res = parse_config(file_path);
        match config_res.and_then(search_for_biggest_files) {
            Ok(_) => (),
            Err(e) => panic!("{:#?}", e),
        };
    }
}

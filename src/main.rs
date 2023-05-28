use std::env;
use std::fs::{self, DirBuilder};
use std::path::{Path};
use serde::Deserialize;

#[derive(Deserialize)]
struct Category {
    extensions: Option<Vec<String>>,
    filenames: Option<Vec<String>>,
}

#[derive(Deserialize)]
struct Config {
    ignore_uncategorized: bool,
    sort_uncategoriezd_by_ext: bool,
    uncategorized_dir: String,
    no_extension_dir: String,
    categories: std::collections::HashMap<String, Category>,
}

fn main() {
    let config_path = env::current_dir().unwrap().join("short.json");

    if !config_path.exists() {
        print!("Creating default config short.json");

        fs::write(&config_path, r#"{
    "ignore_uncategorized": false,
    "sort_uncategoriezd_by_ext": true,
    "uncategorized_dir": "",
    "no_extension_dir": "NoExtension",
    "categories": {}
}"#).expect("Cannot create default config short.json (insufficient permissions?)");
    }

    let config_file = fs::read_to_string(config_path).expect("Cannot read short.json (maybe the file doesn't exist?)");
    let config: Config = serde_json::from_str(&config_file).expect("File short.json is invalid");

    let args: Vec<String> = env::args().collect();

    let current_dir = if args.len() == 2 {
        &args[1]
    } else {
        "."
    };

    let files = fs::read_dir(&current_dir).expect("Cannot read CWD (insufficient permissions?)");

    for file in files {
        if let Ok(file) = file {
            let file_path = file.path();

            if !file_path.is_file() {
                continue;
            }

            let extension = file_path.extension().unwrap_or_default().to_str().unwrap().to_lowercase();

            if file_path.file_name().unwrap() == "short.json" {
                continue;
            }

            if let Some(cat) = get_category(&config, &extension, &file_path) {
                // Categorized
                move_file(&file_path, &current_dir, &cat);
            } else if !config.ignore_uncategorized {
                if config.sort_uncategoriezd_by_ext {
                    if extension != "" {
                        // Uncategorized: move to the uncategorized dir
                        move_file(&file_path, &current_dir, Path::new(&config.uncategorized_dir).join(&extension).as_os_str().to_str().unwrap());
                    } else {
                        // Uncategorized, no extension: move to the 'no extension' dir
                        move_file(&file_path, &current_dir, Path::new(&config.uncategorized_dir).join(&config.no_extension_dir).as_os_str().to_str().unwrap());
                    }
                } else {
                    move_file(&file_path, &current_dir, &config.uncategorized_dir);
                }
            }
        }
    }
}

fn get_category<'config>(config: &'config Config, ext: &str, file_path: &Path) -> Option<&'config str> {
    for (category, value) in &config.categories {
        if let Some(extensions) = &value.extensions {
            if extensions.iter().any(|e| e == ext) {
                return Some(category);
            }
        }

        if let Some(filenames) = &value.filenames {
            if filenames
                .iter()
                .any(|f| f == file_path.file_name().unwrap().to_str().unwrap())
            {
                return Some(category);
            }
        }
    }

    None
}

fn move_file(file: &Path, current_dir: &str, subdir: &str) {
    let dir = Path::new(current_dir).join(subdir);

    if !dir.exists() {
        DirBuilder::new()
            .recursive(true)
            .create(&dir)
            .expect(&format!("Cannot create directory {:?} (maybe it contains illegal characters?)", subdir));
    }

    let new_file = dir.join(file.file_name().unwrap());
    fs::rename(&file, &new_file).expect(&format!("Cannot move file {:?} to directory {:?}", file.file_name().unwrap(), new_file.as_os_str()))
}
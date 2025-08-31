use clap::Parser;
use std::path::{Path, PathBuf};
use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    add: Option<String>,

    #[arg(short, long)]
    remove: Option<String>,

    #[arg(short, long)]
    list: bool,

    /// Jump to a project by index or name
    project: Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
struct Config {
    paths: Vec<String>,
}

fn get_config_path() -> PathBuf {
    let home = dirs::home_dir().expect("Could not find home directory");
    home.join(".teleproj.toml")
}

fn load_config() -> Config {
    let config_path = get_config_path();
    
    if config_path.exists() {
        let content = fs::read_to_string(&config_path)
            .expect("Failed to read config file");
        toml::from_str(&content)
            .unwrap_or_else(|_| {
                eprintln!("Warning: Invalid config file format, using defaults");
                Config::default()
            })
    } else {
        Config::default()
    }
}

fn save_config(config: &Config) {
    let config_path = get_config_path();
    let content = toml::to_string_pretty(config)
        .expect("Failed to serialize config");
    
    fs::write(&config_path, content)
        .expect("Failed to write config file");
}

fn is_valid_path(path: &str) -> bool {
    Path::new(path).exists()
}

fn get_project_name(path: &Path) -> String {
    path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("unknown")
        .to_string()
}

fn add_path(path_str: &str) {
    if !is_valid_path(path_str) {
        eprintln!("Error: The path '{}' does not exist.", path_str);
        std::process::exit(1);
    }

    let canonical_path = fs::canonicalize(path_str)
        .expect("Failed to canonicalize path");
    let path_string = canonical_path.display().to_string();

    let mut config = load_config();
    
    // Check if path already exists
    if config.paths.contains(&path_string) {
        println!("Path already exists: {}", path_string);
        return;
    }

    config.paths.push(path_string.clone());
    save_config(&config);
    println!("Added path: {}", path_string);
}

fn get_paths() -> Vec<PathBuf> {
    let config = load_config();
    config.paths
        .iter()
        .map(|p| PathBuf::from(p))
        .filter(|path| path.exists()) 
        .collect()
}

fn remove_path_by_index(index: usize) {
    let mut config = load_config();
    
    if index < config.paths.len() {
        let removed = config.paths.remove(index);
        save_config(&config);
        println!("Removed path: {}", removed);
    } else {
        eprintln!("Error: Index {} is out of range", index);
        std::process::exit(1);
    }
}

fn calculate_match_score(project_name: &str, query: &str) -> i32 {
    let project_lower = project_name.to_lowercase();
    let query_lower = query.to_lowercase();
    
    if project_lower == query_lower {
        return 1000;
    }
    
    if project_lower.starts_with(&query_lower) {
        return 500 + (query.len() as i32 * 10);
    }
    
    if project_lower.contains(&query_lower) {
        return 100 + (query.len() as i32 * 5);
    }
    
    let mut score = 0;
    let mut query_chars = query_lower.chars();
    let mut current_char = query_chars.next();
    
    for c in project_lower.chars() {
        if let Some(query_char) = current_char {
            if c == query_char {
                score += 1;
                current_char = query_chars.next();
            }
        }
    }
    
    if current_char.is_none() {
        score
    } else {
        0
    }
}

fn find_and_print_path(query: &str) {
    let paths = get_paths();
    
    if let Ok(index) = query.parse::<usize>() {
        if let Some(path) = paths.get(index) {
            println!("{}", path.display());
            return;
        }
    }
    
    let mut matches: Vec<(usize, &PathBuf, i32)> = paths
        .iter()
        .enumerate()
        .filter_map(|(i, path)| {
            let project_name = get_project_name(path);
            let score = calculate_match_score(&project_name, query);
            if score > 0 {
                Some((i, path, score))
            } else {
                None
            }
        })
        .collect();
    
    matches.sort_by(|a, b| b.2.cmp(&a.2));
    
    match matches.len() {
        0 => {
            eprintln!("Error: No project found matching '{}'", query);
            std::process::exit(1);
        }
        1 => {
            println!("{}", matches[0].1.display());
        }
        _ => {
            eprintln!("Multiple projects match '{}'. Please choose:", query);
            for (i, (original_index, path, _score)) in matches.iter().take(5).enumerate() {
                let project_name = get_project_name(path);
                eprintln!("  {}: {} (index {})", i, project_name, original_index);
            }
            eprintln!("\nUse the specific index number to jump to a project.");
            std::process::exit(1);
        }
    }
}

fn main() {
    let args = Args::parse();

    if let Some(add_path_str) = args.add.as_ref() {
        add_path(add_path_str.as_str());
        return;
    }

    if let Some(remove_index) = args.remove.as_ref() {
        match remove_index.parse::<usize>() {
            Ok(index) => remove_path_by_index(index),
            Err(_) => {
                eprintln!("Error: Remove argument must be a valid number");
                std::process::exit(1);
            }
        }
        return;
    }

    if args.list {
        let paths = get_paths();
        if paths.is_empty() {
            println!("No paths saved yet. Use --add to add some!");
        } else {
            println!("Saved projects:");
            for (i, path) in paths.iter().enumerate() {
                let project_name = get_project_name(path);
                println!("{}: {} ({})", i, project_name, path.display());
            }
        }
        return;
    }

    if let Some(project) = args.project.as_ref() {
        find_and_print_path(project);
        return;
    }

    println!("Use --help for usage information");
}
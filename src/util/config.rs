use lazy_static::lazy_static;
use log::{error, info, warn};
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::string::ToString;
use std::{env, fs};

lazy_static! {
    pub static ref CONFIG_DIR : String = env::var("CONFIG_DIR").unwrap_or("config/".to_string());
}

pub fn get_config<T: for<'a> Deserialize<'a>>(path: PathBuf) -> T {
    let verified_path = verify_path_or_copy_default_into_path(path);
    let config: T = serde_json::from_reader(BufReader::new(File::open(verified_path).unwrap())).unwrap();

    config
}

pub fn verify_path_or_copy_default_into_path(path: PathBuf) -> PathBuf {
    if path.try_exists().unwrap() {
        info!("Loading config from {:?}", path);
        return path;
    }

    warn!("Could not find config file at {:?}", path);
    let file_name = path.file_name().unwrap().to_str().unwrap();
    let default_file = build_path(vec!["default-config", file_name]);
    warn!("Try to copy default config file into this path from {:?}", default_file);
    match copy_with_dir_creation(&default_file, &path) {
        Ok(_) => warn!("Copied default config file successfully."),
        Err(e) => error!("Failed to copy default config file: {}", e),
    }
    path
}

pub fn build_path(segments: Vec<&str>) -> PathBuf {
    let mut path = PathBuf::new();
    for segment in segments {
        path.push(segment);
    }
    path
}

fn copy_with_dir_creation(from: &PathBuf, to: &PathBuf) -> anyhow::Result<()> {
    // Get the parent directory of the destination path
    if let Some(parent) = to.parent() {
        // Create the directory and all parent directories if they don't exist
        fs::create_dir_all(parent)?;
    }

    fs::copy(from, to)?;
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::relative_path_trailing_slash(vec!["config/", "my.config.json"], "config/my.config.json"
    )]
    #[case::relative_path_no_trailing_slash(vec!["config", "my.config.json"], "config/my.config.json"
    )]
    #[case::absolute_path_trailing_slash(vec!["/app/data/config/", "my.config.json"], "/app/data/config/my.config.json"
    )]
    #[case::absolute_path_no_trailing_slash(vec!["/app/data/config", "my.config.json"], "/app/data/config/my.config.json"
    )]
    #[case::absolute_path_multi_directory(vec!["/app", "data", "config", "my.config.json"], "/app/data/config/my.config.json"
    )]
    fn should_build_path(#[case] segments: Vec<&str>, #[case] expected_path: String) {
        let actual = build_path(segments);

        assert_eq!(actual.to_str().unwrap(), expected_path);
    }
}
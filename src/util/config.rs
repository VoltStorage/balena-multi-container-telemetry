use lazy_static::lazy_static;
use log::info;
use serde::Deserialize;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::string::ToString;

lazy_static! {
    pub static ref CONFIG_DIR : String = env::var("CONFIG_DIR").unwrap_or("config/".to_string());
}

pub fn get_config<T: for<'a> Deserialize<'a>>(path: PathBuf) -> T {
    info!("Loading config from {:?}", path);
    let config: T = serde_json::from_reader(BufReader::new(File::open(path).unwrap())).unwrap();

    config
}

pub fn build_path(segments: Vec<&str>) -> PathBuf {
    let mut path = PathBuf::new();
    for segment in segments {
        path.push(segment);
    }
    path
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
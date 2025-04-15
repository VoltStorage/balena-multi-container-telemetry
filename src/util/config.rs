use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;

pub fn get_config<T: for<'a> Deserialize<'a>>(path: String) -> T {
    let config: T = serde_json::from_reader(BufReader::new(File::open(path).unwrap())).unwrap();

    config
}

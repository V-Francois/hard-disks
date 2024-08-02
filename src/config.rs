use serde::{Deserialize, Serialize};
use serde_yaml::{self};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub n_disk: u32,
    pub packing_fraction: f64,
    pub n_step: u32,
    pub pressure: Option<f64>,
}

impl Config {
    pub fn from_yaml_file(file_path: &str) -> Config {
        let f = std::fs::File::open(file_path).expect("Could not open file.");
        let config: Config = serde_yaml::from_reader(f).expect("Could not read values.");
        return config;
    }
}

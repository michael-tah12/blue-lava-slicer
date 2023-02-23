use nalgebra::Vector3;
use serde_derive::Deserialize;
use std::fs;
use toml;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub general: General,
    pub quality: Quality,
}

#[derive(Debug, Deserialize)]
pub struct General {
    pub placement: [f64; 3],
    pub filament_diameter: f64,
}

#[derive(Debug, Deserialize)]
pub struct Quality {
    pub line_width: f64,
    pub layer_height: f64,
}

pub fn read_config_toml(filename: &str) -> Config {
    let content = fs::read_to_string(filename).expect("");
    let data: Config = toml::from_str(&content).unwrap();
    return data;
}

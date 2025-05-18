use std::fs;
use std::collections::HashMap;

const CONFIG_FILE_PATH: &str = "./src/config.ron";

#[derive(serde::Deserialize)]
pub struct Node{
    pub nodes: Option<Vec<String>>,
    pub on_click: Option<String>,
    pub text: Option<String>,
    pub icon: Option<String>,
}

pub fn fetch_config() -> HashMap<String, Node>{
    let config_file: String = fs::read_to_string(CONFIG_FILE_PATH).unwrap();

    let config: HashMap<String, Node> = ron::from_str(&config_file).unwrap();
    config
}
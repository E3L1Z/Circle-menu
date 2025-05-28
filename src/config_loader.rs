use std::fs;
use std::collections::HashMap;

const CONFIG_FILE_PATH: &str = "./src/config.ron";


#[derive(serde::Deserialize)]
pub struct Config{
    pub general: General,
    pub nodes: HashMap<String, Node>
}


#[derive(serde::Deserialize)]
pub struct General{
    pub button_layout: String,
    pub fill_wheel: bool,
    pub max_child_node_count: i8,
}


#[derive(serde::Deserialize)]
pub struct Node{
    pub nodes: Vec<String>,
    pub on_click: String,
    pub text: String,
    pub icon: String,
}

pub fn fetch_config() -> Config{
    //let user_dir = format!(
    //    "{}/.config/anyrun",
    //    env::var("HOME").expect("Could not determine home directory! Is $HOME set?")
    //);

    let config_file: String = fs::read_to_string(CONFIG_FILE_PATH).unwrap();

    let config: Config = ron::from_str(&config_file).unwrap();
    config
}
use std::fs;
use std::collections::HashMap;

const CONFIG_FILE_PATH: &str = "./src/config.ron";


#[derive(serde::Deserialize)]
pub struct Config{
    pub general: General,
    pub nodes: HashMap<String, Node>
}

impl Default for Config{
    fn default() -> Self {
        Self{
            general: General::default(),
            nodes: HashMap::from([(String::from("root"), Node::default())])
        }
    }
}


#[derive(serde::Deserialize)]
pub struct General{
    pub button_layout: String,
    pub fill_wheel: bool,
    pub max_child_node_count: i8,
    pub wheel_size: f64,
    pub sub_wheel_distance: f64,
}

impl Default for General{
    fn default() -> Self {
        Self{
            button_layout: String::from("Vertical"),
            fill_wheel: true,
            max_child_node_count: 8,
            wheel_size: 200.0,
            sub_wheel_distance: 1.7,
        }
    }
}


#[derive(serde::Deserialize)]
pub struct Node{
    pub nodes: Vec<String>,
    pub on_click: String,
    pub text: String,
    pub icon: String,
}

impl Default for Node{
    fn default() -> Self {
        Self{
            nodes: Vec::new(),
            on_click: String::new(),
            text: String::new(),
            icon: String::new(),
        }
    }
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
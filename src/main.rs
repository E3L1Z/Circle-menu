use std::collections::HashMap;

mod root_node;
mod config_loader;

fn main() {
    let config: HashMap<String, config_loader::Node> = config_loader::fetch_config();

    let root = root_node::create_node("root", &config);
    
    //print!("{on_click}", on_click=root.child_nodes[3].on_click);
    root.child_nodes[3].on_click(&config);
}
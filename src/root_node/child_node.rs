use std::{collections::HashMap, process::Command};
use crate::root_node;
use crate::config_loader;

pub struct ChildNode{
    id: String,
    pub on_click: String,
}

impl ChildNode {
    pub fn initialize(_id: String, _on_click: String) -> Self{
        ChildNode {
            id: _id,
            on_click: _on_click
        }
    }

    pub fn on_click(&self, config:&HashMap<String, config_loader::Node>) -> Option<root_node::RootNode>{
        let own_config = config.get(&self.id).expect(&format!("{id} is not defined", id=self.id));
        if own_config.nodes != None && own_config.nodes.as_ref().unwrap().len() != 0{
            return Some(root_node::create_node(&self.id, config))
        }

        if self.on_click == "" {return None} 

        Command::new("sh").arg("-c").arg(&self.on_click).output().expect("failed to execute process");

        None
    }
}
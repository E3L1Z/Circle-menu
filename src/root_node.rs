use std::collections::HashMap;
use crate::config_loader;

pub mod child_node;

pub struct RootNode{
    id: String,
    pub child_nodes: Vec<child_node::ChildNode>
    
}

impl RootNode{
    pub fn initialize(_id:String, _child_nodes: Vec<child_node::ChildNode>) -> Self{
        Self {
            id: _id,
            child_nodes: _child_nodes,
        }
    }

    pub fn on_click(&self){
        //TODO
        //Close folder node
    }
}

pub fn create_node(id: &str, config: &HashMap<String, config_loader::Node>) -> RootNode{
    let node = config.get("root").expect("Root not defined");

    let mut child_nodes = Vec::new();
    for child in node.nodes.iter().flatten(){
        let child_config = config.get(child).expect(&format!("{child} is not defined"));

        child_nodes.push(child_node::ChildNode::initialize(child.clone(), child_config.on_click.clone().unwrap_or(String::new())));
    }

    RootNode::initialize(String::from(id), child_nodes)
}
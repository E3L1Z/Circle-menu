use gtk4 as gtk;

pub struct RootNode{
    pub id: String,
    pub child_nodes: Vec<ChildNode>,
    pub button: gtk::Button,
}

impl RootNode{
    pub fn initialize(_id:String, _child_nodes: Vec<ChildNode>, _button: gtk::Button) -> Self{
        Self {
            id: _id,
            child_nodes: _child_nodes,
            button: _button,
        }
    }
}


pub struct ChildNode{
    pub id: String,
    pub on_click: String,
    pub button: gtk::Button,
}


impl ChildNode {
    pub fn initialize(_id: String, _on_click: String, _button: gtk::Button) -> Self{
        Self {
            id: _id,
            on_click: _on_click,
            button: _button,
        }
    }
}
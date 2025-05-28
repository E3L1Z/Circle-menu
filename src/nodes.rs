use gtk4 as gtk;

pub struct RootNode{
    pub id: String,
    pub child_nodes: Vec<ChildNode>,
    pub button_container: ButtonContainer,
}

impl RootNode{
    pub fn initialize(_id:String, _child_nodes: Vec<ChildNode>, _button_container: ButtonContainer) -> Self{
        Self {
            id: _id,
            child_nodes: _child_nodes,
            button_container: _button_container,
        }
    }

    pub fn on_click(&self){
        //TODO
        //Close folder node
    }
}


pub struct ChildNode{
    pub id: String,
    pub on_click: String,
    pub button_container: ButtonContainer,
}


pub struct ButtonContainer{
    container: gtk::Box,
    pub button: gtk::Button,
    label: gtk::Label,
}


impl ButtonContainer{
    pub fn initialize(_container: gtk::Box, _button: gtk::Button, _label: gtk::Label) -> Self{
        Self {
            container: _container,
            button: _button,
            label: _label,
        }
    }
}


impl ChildNode {
    pub fn initialize(_id: String, _on_click: String, _button_container: ButtonContainer) -> Self{
        Self {
            id: _id,
            on_click: _on_click,
            button_container: _button_container,
        }
    }
}
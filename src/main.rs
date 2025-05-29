use core::f64;
use gtk4 as gtk;
use gtk::prelude::*;
use gdk;
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use std::{process::Command, cell::RefCell, rc::Rc};
use glib::clone;

mod nodes;
mod config_loader;

const SOFTWARE_NAME: &str = "Circle-menu";
const STYLE_FILE_PATH: &str = "./src/style.css";


fn activate(application: &gtk::Application) {
    //let window = gtk::ApplicationWindow::new(application);
    let window = gtk::ApplicationWindow::builder().application(application).title(SOFTWARE_NAME).build();

    window.init_layer_shell();
    window.set_layer(Layer::Overlay);

    let anchors = [
        (Edge::Left, true),
        (Edge::Right, true),
        (Edge::Top, true),
        (Edge::Bottom, true),
    ];

    for (anchor, state) in anchors {
        window.set_anchor(anchor, state);
    }

    let fixed = gtk::Fixed::builder().build();

    let config: Rc<config_loader::Config> = Rc::new(config_loader::fetch_config());

    let x_pos = 1920.0 / 2.0;
    let y_pos = 1080.0 / 2.0;

    let root = create_node("root", Rc::clone(&config), &x_pos, &y_pos, &fixed, &window);
    window.set_child(Some(&fixed));

    window.present();
}


fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = gtk::CssProvider::new();
    provider.load_from_file(&gio::File::for_path(std::path::Path::new(STYLE_FILE_PATH)));

    // Add the provider to the default screen
    gtk::style_context_add_provider_for_display(
        &gdk::Display::default().expect("Could not connect to a display."), &provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}


fn create_button_container(x: &f64, y: &f64, fixed: &gtk::Fixed, config: Rc<config_loader::Config>, id: &str) -> gtk::Button{
    let own_config = config.nodes.get(id).expect(&format!("{} was not specified in config", id));
    
    let orientation: gtk::Orientation;
    match config.general.button_layout.as_str(){
        "Horizontal" => orientation = gtk::Orientation::Horizontal,
        "Vertical" => orientation = gtk::Orientation::Vertical,
        _ => panic!("Button orientation was not specified correctly"),
    }

    let container = gtk::Box::builder().orientation(orientation).build();
    fixed.put(&container, x.clone(), y.clone());

    let button = gtk::Button::builder().build();
    button.add_css_class(&format!("{}_button", id));

    container.append(&button);

    let label = gtk::Label::builder().label(&own_config.text).build();
    label.add_css_class(&format!("{}_label", id));
    container.append(&label);
    
    if !own_config.icon.is_empty() {
        let picture = gtk::Picture::for_file(&gio::File::for_path(std::path::Path::new(&own_config.icon)));

        button.set_child(Some(&picture));
    }

    button
}


fn create_node(id: &str, config: Rc<config_loader::Config>, x: &f64, y: &f64, fixed: &gtk::Fixed, window: &gtk::ApplicationWindow) -> nodes::RootNode{
    let node = config.nodes.get(id).expect("Root is not defined");

    let mut child_nodes = Vec::new();
    for (index, child) in node.nodes.iter().enumerate(){
        if index >= (config.general.max_child_node_count - 1) as usize { break; }
        
        let child_config = config.nodes.get(child).expect(&format!("{child} is not defined"));

        let mut angle = (f64::consts::PI * 2.0) / (config.general.max_child_node_count as f64) * (index as f64);
        if config.general.fill_wheel { angle = (f64::consts::PI * 2.0) / (node.nodes.len() as f64) * (index as f64); }

        let wheel_size = config.general.wheel_size;

        let child_x = f64::sin(angle) * wheel_size + x;
        let child_y = f64::cos(angle) * -1.0 * wheel_size + y;

        let child_container = create_button_container(&child_x, &child_y, fixed, Rc::clone(&config), child);

        let child_node = nodes::ChildNode::initialize(child.clone(), child_config.on_click.clone(), child_container);

        if child_config.nodes.len() > 0{
            child_node.button.connect_clicked({
                let id_clone: String = String::from(child);
                let sub_wheel_distance = config.general.sub_wheel_distance;
                let x = child_x + (child_x - x) * sub_wheel_distance;
                let y = child_y + (child_y - y) * sub_wheel_distance;
                clone!(#[strong] config, #[strong] fixed, #[strong] window, move |_| {
                    create_node(&id_clone, Rc::clone(&config), &x, &y, &fixed, &window);
                })
            });
        } else {
            child_node.button.connect_clicked({
                let button_on_click = child_node.on_click.clone();
                clone!(#[strong] window, move |_| {
                    Command::new("sh").arg("-c").arg(&button_on_click).spawn().expect("failed to execute process");
                    window.close();
            })});
        }

        child_nodes.push(child_node);
    }

    let root = nodes::RootNode::initialize(String::from(id), child_nodes, create_button_container(x, y, fixed, config, id));
    root
}


fn main() {
    //let application = gtk::Application::new(Some("sh.wmww-Test"), Default::default());
    let application = gtk::Application::builder().application_id(format!("com.{}", SOFTWARE_NAME)).build();

    if application.is_remote() {
        return;
    }

    application.connect_startup(|_| {load_css()});
    application.connect_activate(|app| { activate(app); });

    application.run();
}
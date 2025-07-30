mod styling;
mod contents;
use gtk::prelude::*;
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use styling::colors::{run_pywal, read_pywal_colors, darkest_and_lightest};
use styling::theme::{generate_css,apply_theme};
use contents::layout::{ generate_bar };
use contents::panel:: { Panel };
fn main() {
    
    let app = gtk::Application::new(Some("app.wswitch"), Default::default());

    app.connect_activate(|app| {
        build_window(app)
    });

    change_theme();
    app.run();
}

fn build_window (app: &gtk::Application) {

    let window = gtk::ApplicationWindow::new(app);
    window.init_layer_shell(); 
    window.set_layer(Layer::Overlay);
    window.auto_exclusive_zone_enable();

    window.set_margin(Edge::Left, 0);
    window.set_margin(Edge::Right, 0);
    window.set_margin(Edge::Top, 0);
    
    let anchors = [
        (Edge::Left, true),
        (Edge::Right, true),
        (Edge::Top, true),
        (Edge::Bottom, false)

    ];

    for (anchor, state) in anchors {
        window.set_anchor(anchor, state);
    }

    let label = gtk::Label::new(Some(""));
    label.set_markup("<span font_desc=\"20.0\">GTK Layer Shell example!</span>");
    //window.set_child(Some(&label));
    let bar = generate_bar();
    window.set_child(Some(&bar));

    let panel = Panel::new(app);
    panel.set_hover_area(&bar);
    window.show();
}

fn change_theme () {
     run_pywal("/home/kronos/Pictures/red-sun.jpg");

    let colors = read_pywal_colors();
    println!("HOME is: {:?}", std::env::var("HOME"));
    println!("{:#?}",&colors);

    let (darkest, lightest) = darkest_and_lightest(&colors.colors);
    let css = generate_css("Iosevka","flat-dark", &darkest, &lightest, &colors.special.background, &colors.special.foreground);
    
    gtk::init().unwrap();
    apply_theme(&css);
}

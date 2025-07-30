use gtk::prelude::*;
use gtk4_layer_shell::{Edge, Layer, LayerShell};


fn main() {
    
    let app = gtk::Application::new(Some("app.wswitch"), Default::default());

    app.connect_activate(|app| {
        build_window(app)
    });
    app.run();
}

fn build_window (app: &gtk::Application) {

    let window = gtk::ApplicationWindow::new(app);
    window.init_layer_shell(); 
    window.set_layer(Layer::Overlay);
    window.auto_exclusive_zone_enable();

    window.set_margin(Edge::Left, 40);
    window.set_margin(Edge::Right, 40);
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
    window.set_child(Some(&label));
    window.show();
}

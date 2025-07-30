use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, CenterBox, Box as GtkBox, Label};

pub fn generate_bar () -> CenterBox {
    
    let left = gtk::Label::new(Some("Workspaces"));
    left.set_margin_start(10);
    let center = gtk::Label::new(Some("Center Widget"));

    let right = GtkBox::new(gtk::Orientation::Horizontal, 10);
    right.append(&Label::new(Some("CPU")));
    right.append(&Label::new(Some("Ram")));
    right.append(&Label::new(Some("12:45")));
    right.set_margin_end(10);


    let bar = CenterBox::new();
    bar.set_start_widget(Some(&left));
    bar.set_center_widget(Some(&center));
    bar.set_end_widget(Some(&right));

    bar


}

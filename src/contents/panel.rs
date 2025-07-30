use gtk::prelude::*;
use gtk::{ApplicationWindow, Button, Box as GtkBox, Label};
use gtk4_layer_shell::{self as layer_shell, Edge, Layer, LayerShell};
use glib::clone;

pub struct Panel {
    panel: ApplicationWindow,
    arrow: ApplicationWindow,
}


impl Panel {
    
    pub fn new(app: &gtk::Application) -> Self {

        let panel = ApplicationWindow::new(app);
        panel.init_layer_shell();
        panel.set_layer(Layer::Overlay);
        panel.set_exclusive_zone(-1);
        panel.set_margin_top(50);
        panel.set_anchor(Edge::Top, true);
        panel.set_default_size(400, 300);
        panel.set_opacity(1.0);
        panel.hide();

        let arrow = ApplicationWindow::new(app);
        arrow.init_layer_shell();
        arrow.set_layer(Layer::Overlay);
        arrow.set_anchor(Edge::Top, true);
        arrow.set_margin_top(50);
        arrow.hide();

        let arrow_btn = Button::with_label("▼");
        arrow_btn.set_opacity(0.0);
        arrow.set_child(Some(&arrow_btn));

        let content = gtk::Grid::new();
        content.set_row_spacing(10);
        content.set_column_spacing(20);
        content.set_margin_top(20);
        content.set_margin_start(20);
        content.attach(&Label::new(Some("Settings")), 0, 0, 1, 1);
        content.attach(&Button::with_label("Option 1"), 0, 1, 1, 1);
        content.attach(&Button::with_label("Option 2"), 1, 1, 1, 1);
        content.attach(&Button::with_label("Option 3"), 0, 2, 1, 1);
        content.attach(&Button::with_label("Option 4"), 1, 2, 1, 1);


        panel.set_child(Some(&content));

        let panel_clone = panel.clone();
        arrow_btn.connect_clicked(move |_| {
            if panel_clone.is_visible() {
                panel_clone.hide();
            } else {
                panel_clone.show();
                panel_clone.present();
            }
        });

        Self { panel, arrow }
    }


    pub fn set_hover_area<W: IsA<gtk::Widget>>(&self, bar_window: &W) {
        let arrow = self.arrow.clone();
        let bar_window_clone = bar_window.clone().upcast::<gtk::Widget>();
        let gesture = gtk::EventControllerMotion::new();

        gesture.connect_motion( move |_, x, _| {
            let width = bar_window_clone.allocated_width();
            let center = (width / 2) as f64;

            if (x - center).abs() < 50.0 {
                arrow.show();
            } else {
                arrow.hide();
            }
        });

        bar_window.add_controller(gesture);
    }



            
}

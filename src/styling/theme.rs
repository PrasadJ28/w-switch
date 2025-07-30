use gtk::prelude::*;
use gtk::{CssProvider, gdk};

pub fn generate_css(font_type: &str,theme_type: &str, darkest: &str, lightest: &str, base_bg: &str, fg: &str) -> String {
    match theme_type {
        "flat-dark" => format!(r#"
            window, headerbar, .bar {{
                font-family: {font}, monospace;
                background: {dark};
                color: white;
            }}
        "#, dark = darkest, font = font_type),
        "flat-light" => format!(r#"
            window, headerbar, .bar {{
                font-family: {font}, monospace;
                background: {light};
                color: black;
            }}
        "#, light = lightest, font = font_type),
        "translucent" => format!(r#"
            window, headerbar, .bar {{
                font-family: {font}, monospace;
                background: rgba(0,0,0,0.6);
                color: {fg};
                backdrop-filter: blur(10px);
            }}
        "#, fg = fg, font = font_type),
        "transparent" => format!(r#"
            window, headerbar, .bar {{
                font-family: {font}, monospace;
                background: transparent;
                color: {fg};
            }}
        "#, fg = fg, font = font_type),
        _ => "".to_string(),
    }
}


pub fn apply_theme(css: &str) {
    let provider = CssProvider::new();
    provider.load_from_data(&css);

    gtk::style_context_add_provider_for_display(
    &gtk::gdk::Display::default().unwrap(),
    &provider,
    gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
);

}


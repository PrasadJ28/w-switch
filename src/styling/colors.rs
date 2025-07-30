use std::process::Command;
use serde::Deserialize;
use std::fs;


#[derive(Debug, Deserialize)]
pub struct PywalColors {
    pub special: Special,
    pub colors: std::collections::HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct Special {
    pub background: String,
    pub foreground: String,
    pub cursor: String,
}

pub fn run_pywal(wallpaper: &str) {
    Command::new("wal")
        .args(&["-i", wallpaper, "-n"])
        .status()
        .expect("Failed to run pywal");
}

pub fn read_pywal_colors() -> PywalColors {
    let data = fs::read_to_string(format!("{}/.cache/wal/colors.json", std::env::var("HOME").unwrap()))
        .expect("Could not read colors.json");
    serde_json::from_str(&data).expect("Invalid JSON")
}

fn hex_to_rgb(hex: &str) -> (u8, u8, u8) {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
    (r, g, b)
}

fn luminance((r, g, b): (u8, u8, u8)) -> f64 {
    0.2126 * (r as f64 / 255.0) + 0.7152 * (g as f64 / 255.0) + 0.0722 * (b as f64 / 255.0)
}

pub fn darkest_and_lightest(colors: &std::collections::HashMap<String, String>) -> (String, String) {
    let mut sorted: Vec<_> = colors.values().cloned().collect();
    sorted.sort_by(|a, b| {
        luminance(hex_to_rgb(a)).partial_cmp(&luminance(hex_to_rgb(b))).unwrap()
    });
    (sorted[0].clone(), sorted.last().unwrap().clone())
}

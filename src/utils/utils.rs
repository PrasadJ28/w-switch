use gdk::{ prelude::{DisplayExt, MonitorExt}, Display};
use gtk::prelude::*;


pub fn get_monitor_size() -> (i32, i32) {
    
    let display = Display::default().expect("No Display found");
    let monitors = display.monitors();
    if let Some(obj) = monitors.item(0){
        if let Ok(monitor) = obj.downcast::<gdk::Monitor>() {
            let geometry = monitor.geometry();
            return (geometry.width(), geometry.height());
        }
    } 

    (1920, 1080)

}

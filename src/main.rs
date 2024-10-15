#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cfg;
mod components;
mod md;
mod omega;
mod utils;

// blitz example
// use dioxus_core::VirtualDom;
// use dioxus_blitz::exports::dioxus::prelude::*;
// use dioxus_blitz::{DioxusDocument, WindowConfig};

// fn main() {
//     let dom = VirtualDom::new(app);
//     let doc = DioxusDocument::new(dom);
//     let window = WindowConfig::new(doc, 1200.0, 700.0);
//     dioxus_blitz::launch_with_window(window);
//     LaunchBuilder::new().with_cfg(config()).launch(app);
//     launch_desktop(app);
// }

use dioxus::desktop::{Config, LogicalPosition, LogicalSize, WindowBuilder};
use dioxus::prelude::*;

fn main() {
    LaunchBuilder::new()
        .with_cfg(config())
        .launch(components::app);
}

fn config() -> Config {
    use dioxus::desktop::tao::window::Icon;
    let img = image::load_from_memory(include_bytes!("../logo.png")).unwrap().to_rgba8();
    let rgba = img.into_raw();
    let icon = Icon::from_rgba(rgba, 256, 256).unwrap();

    Config::default().with_window(
        WindowBuilder::new()
            .with_title("Omega AI")
            .with_window_icon(Some(icon))
            .with_position(LogicalPosition::new(0, 0))
            .with_resizable(true)
            .with_decorations(false)
            .with_inner_size(LogicalSize::new(1200, 750))
            .with_min_inner_size(LogicalSize::new(800, 500)),
    )
}

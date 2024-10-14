use dioxus::prelude::*;

use crate::cfg::Cfg;
use crate::utils;

use super::svg;

pub fn app() -> Element {
    let cfg = use_signal(Cfg::load);
    let current_chat = use_signal(utils::get_time);

    rsx!(
        // head::Link {
        //     rel: "stylesheet",
        //     href: asset!("./src/style.css")
        // }
        // head::Link {
        //     rel: "stylesheet",
        //     href: asset!("./src/normalize.css")
        // }
        link {
            rel: "stylesheet",
            href: "./assets/css/style.css"
        }
        link {
            rel: "stylesheet",
            href: "./assets/css/normalize.css"
        }
        link {
            rel: "stylesheet",
            href: "./assets/css/katex.css"
        }
        div { class: "app",
            div { class: "header",
                div { class: "logo",
                    onmousedown: move |_| {
                        dioxus::desktop::window().drag();
                    },
                    svg::Omega {}
                }
                div { class: "search-bar",
                    input { r#type: "text", placeholder: "Search..." }
                }
                div { class: "control-area",
                    div { class: "dark-light",
                        svg::Moon {}
                    }
                    div { class: "setting",
                        svg::Setting {}
                    }
                    div { class: "colors",
                        div { "data-color": "blue", class: "color blue selected" }
                        div { "data-color": "purple", class: "color purple" }
                        div { "data-color": "green", class: "color green" }
                        div { "data-color": "orange", class: "color orange" }
                    }
                    div { class: "close",
                        onclick: move |_| {
                            dioxus::desktop::window().close();
                        },

                        svg::Close {}
                    }
                }
            }
            div { class: "wrapper",
                super::history::ChatHistory {
                    cfg: cfg,
                    current_chat: current_chat
                }
                super::chat::Chat {
                    cfg: cfg,
                    current_chat: current_chat
                }
            }
        }
    )
}

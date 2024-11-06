use dioxus::prelude::*;

use crate::cfg::Cfg;
use crate::utils;

use super::svg;

pub fn app() -> Element {
    let mut cfg = use_signal(Cfg::load);
    let current_chat = use_signal(utils::get_time);
    let mut api_key = use_signal(String::new);

    rsx!(
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
            class: if cfg.read().dark_mode { "dark-mode" } else { "" },
            "data-theme": cfg.read().theme.to_str(),
            div { class: "header",
                div { class: "logo",
                    onmousedown: move |_| {
                        dioxus::desktop::window().drag();
                    },
                    svg::Omega {}
                }
                div { class: "search-bar",
                    input {
                        value: "{api_key.read()}",
                        placeholder: "Enter API Key and Click Setting",
                        oninput: move |e| {
                            let value = e.value();
                            api_key.set(value);
                        },
                    }
                }
                div { class: "control-area",
                    div { class: "dark-light",
                        onclick: move |_| {
                            cfg.write().toggle_dark_light();
                        },
                        svg::Moon {}
                    }
                    div { class: "setting",
                        onclick: move |_| {
                            cfg.write().set_key(&api_key.read());
                            api_key.write().clear();
                        },
                        svg::Setting {}
                    }
                    div { class: "colors",
                        div { class: "color blue",
                            class: if cfg.read().theme.to_str() == "blue" { "selected" } else { "" },
                            onclick: move |_| {
                                cfg.write().set_theme("blue");
                            }
                        }
                        div { class: "color purple",
                            class: if cfg.read().theme.to_str() == "purple" { "selected" } else { "" },
                            onclick: move |_| {
                                cfg.write().set_theme("purple");
                            }
                        }
                        div { class: "color green",
                            class: if cfg.read().theme.to_str() == "green" { "selected" } else { "" },
                            onclick: move |_| {
                                cfg.write().set_theme("green");
                            }
                        }
                        div { class: "color orange",
                            class: if cfg.read().theme.to_str() == "orange" { "selected" } else { "" },
                            onclick: move |_| {
                                cfg.write().set_theme("orange");
                            }
                        }
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

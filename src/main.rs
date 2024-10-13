mod cfg;
mod md;
mod omega;
mod svg;
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

use dioxus::prelude::*;
use dioxus::desktop::{Config, WindowBuilder, LogicalSize, LogicalPosition};

use futures::StreamExt;

use cfg::{Cfg, ChatItem};
use omega::{Omega, Role, Message};

fn main() {
    LaunchBuilder::new().with_cfg(config()).launch(app);
}

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
                ChatHistory {
                    cfg: cfg,
                    current_chat: current_chat
                }
                Chat {
                    cfg: cfg,
                    current_chat: current_chat
                }
            }
        }
    )
}

#[component]
fn ChatHistory(cfg: Signal<Cfg>, current_chat: Signal<u64>) -> Element {
    let history_list = cfg.read();
    let history_list_display = history_list.get_chat().iter().map(|ChatItem { title, summary, date }| {
        let date = *date;
        rsx!(
            div { class: "chat-item",
                class: if *current_chat.read() == date { "active" } else { "" },
                onclick: move |_| {
                    current_chat.set(date);
                },
                div { class: "chat-info",
                    div { class: "chat-title", "{title}" }
                    div { class: "chat-profile",
                        span { class: "chat-summary", "{summary}" }
                        span { class: "chat-data", "{date}" }
                    }
                }
            }
        )
    });
    rsx! (
        div { class: "chat-history",
            {history_list_display}
            button { class: "add",
                onclick: move |_| {
                    current_chat.set(1728815980);
                }
            }
            div { class: "overlay" }
        }
    )
}

#[component]
fn Chat(cfg: Signal<Cfg>, current_chat: Signal<u64>) -> Element {
    let mut omega = Omega::new();

    omega.set_key(cfg.read().get_key());

    omega.load(*current_chat.read());

    let mut msg_signal = use_signal(|| omega.get_msg());

    let msg = msg_signal.read();

    let msg_display = msg.iter().map(|Message {role, content, ..}| {
        match role {
            Role::User => {
                rsx!(
                    div { class: "chat-msg owner",
                        div { class: "chat-msg-profile",
                            div { class: "chat-msg-date", "1.22pm" }
                        }
                        div { class: "chat-msg-content",
                            div { class: "chat-msg-text", "{content}" }
                        }
                    }
                )
            }
            Role::Assistant => {
                rsx!(
                    div { class: "chat-msg",
                        div { class: "chat-msg-profile",
                            div { class: "chat-msg-date", "1.22pm" }
                        }
                        div { class: "chat-msg-content",
                            div { class: "chat-msg-text",
                                dangerous_inner_html: "{md::render_md(content)}"
                            }
                        }
                    }
                )
            }
            Role::System => {rsx!()}
        }
    });

    let mut chat_req = use_signal(String::new);

    let mut chat_res = use_signal(String::new);

    let start_chat = use_coroutine(|mut rx: UnboundedReceiver<String> | {
        async move {
            while let Some(req) = rx.next().await {
                let mut res = omega.chat(&req).await.unwrap();

                while let Some(item) = res.next().await {
                    match item {
                        Ok((message, done)) => {
                            chat_res.write().push_str(&message);
                            if done {
                                omega.push_msg(Role::Assistant, chat_res.read().clone());
                                omega.save(*current_chat.read());
                                msg_signal.write().push(Message::new(Role::Assistant, chat_res.read().clone()));
                                chat_res.write().clear();
                                break;
                            }
                        }
                        Err(e) => {
                            eprintln!("Error while reading stream: {}", e);
                            break;
                        }
                    }
                }
            }
        }
    });

    let chat_res_display = if chat_res.read().is_empty() {
        rsx!()
    } else {
        rsx!(
            div { class: "chat-msg",
                div { class: "chat-msg-profile",
                    div { class: "chat-msg-date", "1.22pm" }
                }
                div { class: "chat-msg-content",
                    div { class: "chat-msg-text",
                        dangerous_inner_html: "{md::render_md(&chat_res.read())}"
                    }
                }
            }
        )
    };

    rsx!(
        div { class: "chat-area",
            id: "chat-area",
            div { class: "chat-area-main",
            {msg_display}
            {chat_res_display}
            }
            div { class: "input-area",
                svg::Picture {}
                svg::Paperclip {}
                input {
                    value: "{chat_req.read()}",
                    placeholder: "Type something here...",
                    oninput: move |e| {
                        let value = e.value();
                        chat_req.set(value);
                    },
                }
                svg::Send {
                    onclick: move |_| {
                        msg_signal.write().push(Message::new(Role::User, chat_req.read().clone()));
                        start_chat.send(chat_req.read().clone());
                        chat_req.write().clear();
                    },
                }
            }
        }
    )
}

fn config() -> Config {
    Config::default()
        .with_window(
            WindowBuilder::new()
            .with_title("Omega AI")
            .with_position(LogicalPosition::new(0, 0))
            .with_resizable(true)
            .with_decorations(false)
            .with_inner_size(LogicalSize::new(1200, 750))
            .with_min_inner_size(LogicalSize::new(800, 500))
        )
}
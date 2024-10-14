use dioxus::prelude::*;

use crate::cfg::Cfg;
use crate::utils;

#[component]
pub fn ChatHistory(cfg: Signal<Cfg>, current_chat: Signal<u64>) -> Element {
    let history_list = cfg.read();
    let history_list_display = history_list.get_chat().iter().map(|item| {
        let date = item.date;
        rsx!(
            div { class: "chat-item",
                class: if *current_chat.read() == date { "active" } else { "" },
                key: date,
                onclick: move |_| {
                    current_chat.set(date);
                },
                div { class: "chat-info",
                    div { class: "chat-title", "{item.title}" }
                    div { class: "chat-profile",
                        span { class: "chat-summary", "{item.summary}" }
                        span { class: "chat-data", "{item.date}" }
                    }
                }
            }
        )
    });

    rsx! (
        div { class: "chat-history",
            div {
                {history_list_display}
            }
            button { class: "add",
                onclick: move |_| {
                    current_chat.set(utils::get_time());
                }
            }
            div { class: "overlay" }
        }
    )
}

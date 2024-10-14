use dioxus::prelude::*;
use dioxus::events::Key;

use futures::StreamExt;

use crate::cfg::Cfg;
use crate::md;
use crate::omega::{Message, Omega, OmegaInput, Role};

use super::svg;

#[component]
pub fn Chat(cfg: Signal<Cfg>, current_chat: Signal<u64>) -> Element {
    let mut omega = Omega::new();
    omega.set_key(cfg.read().get_key());

    let mut msg_signal: Signal<OmegaInput> = use_signal(|| OmegaInput::new());
    use_effect(move || {
        msg_signal.set(OmegaInput::load(*current_chat.read()));
    });

    // msg_signal.with_mut(|m| {
    //     omega.load(*current_chat.read());
    //     *m = omega.get_msg();
    // });

    let msg = msg_signal.read();

    let msg_display = msg
        .messages
        .iter()
        .map(|Message { role, content, .. }| match role {
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
            Role::System => {
                rsx!()
            }
        });

    let mut chat_req = use_signal(String::new);

    let mut chat_res = use_signal(String::new);

    let start_chat = use_coroutine(|mut rx: UnboundedReceiver<String>| async move {
        while let Some(req) = rx.next().await {
            msg_signal.write().push(Role::User, req.clone());
            let mut res = omega.chat(&msg_signal.read()).await.unwrap();

            while let Some(item) = res.next().await {
                match item {
                    Ok((message, done)) => {
                        chat_res.write().push_str(&message);
                        if done {
                            msg_signal
                                .write()
                                .push(Role::Assistant, chat_res.read().clone());
                            msg_signal.read().save(*current_chat.read());
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
                div{
                    onclick: move |_| {
                        if msg_signal.read().messages.is_empty() {
                            cfg.write().add_chat(
                                &chat_req.read(),
                                "Summary",
                                *current_chat.read()
                            );
                        }
                        start_chat.send(chat_req.read().clone());
                        chat_req.write().clear();
                    },
                    onkeydown: move |e| {
                        if e.key() == Key::Enter && !chat_req.read().trim().is_empty() {
                            if msg_signal.read().messages.is_empty() {
                                cfg.write().add_chat(
                                    &chat_req.read(),
                                    "Summary",
                                    *current_chat.read()
                                );
                            }
                            start_chat.send(chat_req.read().clone());
                            chat_req.write().clear();
                        }
                    },
                    svg::Send {}
                }
            }
        }
    )
}

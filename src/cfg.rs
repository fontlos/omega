use serde::{Deserialize, Serialize};

use std::fs::File;
use std::path::Path;

#[derive(Deserialize, Serialize)]
pub struct Cfg{
    key: String,
    dark_mode: bool,
    theme: Color,
    chat: Vec<Chat>,
}

#[derive(Deserialize, Serialize)]
enum Color {
    Blue,
    Purple,
    Green,
    Orange,
}

#[derive(Default, Deserialize, Serialize)]
struct Chat {
    title: String,
    summary: String,
    date: String,
    file: String,
}

impl Cfg {
    fn new() -> Self {
        let cfg = Self {
            key: String::new(),
            dark_mode: false,
            theme: Color::Blue,
            chat: Vec::new(),
        };
        cfg.save();
        cfg
    }

    pub fn load() -> Self {
        if Path::new("./config.json").exists() {
            let file = File::open("./config.json").unwrap();
            serde_json::from_reader(file).unwrap()
        } else {
            Self::new()
        }
    }

    pub fn save(&self) {
        let file = File::create("./config.json").unwrap();
        serde_json::to_writer(file, self).unwrap();
    }

    pub fn get_key(&self) -> &str {
        &self.key
    }

    pub fn set_key(&mut self, key: &str) {
        self.key = key.to_string();
    }

    pub fn set_dark_mode(&mut self, dark_mode: bool) {
        self.dark_mode = dark_mode;
    }

    pub fn set_theme(&mut self, theme: Color) {
        self.theme = theme;
    }

    pub fn add_chat(&mut self, chat: Chat) {
        self.chat.push(chat);
    }
}

#[test]
fn test_cfg() {
    let mut cfg = Cfg::load();
    // cfg.set_key("11111111111111111111111111111111111");
    // cfg.save();
    cfg.set_key("");
    cfg.save();
}
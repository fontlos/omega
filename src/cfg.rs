use serde::{Deserialize, Serialize};

use std::fs::File;
use std::path::Path;

#[derive(Deserialize, Serialize)]
pub struct Cfg {
    key: String,
    pub dark_mode: bool,
    pub theme: Color,
    chat: Vec<ChatItem>,
}

#[derive(Deserialize, Serialize)]
pub enum Color {
    Blue,
    Purple,
    Green,
    Orange,
}

impl Color {
    pub fn to_str(&self) -> &str {
        match self {
            Color::Blue => "blue",
            Color::Purple => "purple",
            Color::Green => "green",
            Color::Orange => "orange",
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ChatItem {
    pub title: String,
    pub summary: String,
    pub date: u64,
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
            let mut cfg: Cfg = serde_json::from_reader(file).unwrap();
            cfg.chat.sort_by(|a, b| b.date.cmp(&a.date));
            cfg
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

    pub fn get_chat(&self) -> &Vec<ChatItem> {
        &self.chat
    }

    pub fn set_key(&mut self, key: &str) {
        self.key = key.to_string();
        self.save();
    }

    pub fn toggle_dark_light(&mut self) {
        self.dark_mode = !self.dark_mode;
        self.save();
    }

    pub fn set_theme(&mut self, theme: &str) {
        let theme = match theme {
            "blue" => Color::Blue,
            "purple" => Color::Purple,
            "green" => Color::Green,
            "orange" => Color::Orange,
            _ => Color::Blue,
        };
        self.theme = theme;
        self.save();
    }

    pub fn add_chat(&mut self, title: &str, summary: &str, date: u64) {
        self.chat.push(ChatItem {
            title: title.to_string(),
            summary: summary.to_string(),
            date,
        });
        self.save();
    }

    pub fn del_chat(&mut self, date: u64) {
        self.chat.retain(|chat| chat.date != date);
        let path = format!("./data/{}.json", date);
        std::fs::remove_file(path).unwrap();
        self.save();
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

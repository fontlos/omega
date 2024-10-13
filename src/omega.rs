use futures::future;
use futures::stream::{Stream, StreamExt};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use std::fs::File;
use std::path::Path;
use std::pin::Pin;

pub struct Omega {
    api_key: String,
    client: Client,
    input: OmegaInput,
}

impl Omega {
    pub fn new() -> Self {
        Self {
            api_key: String::new(),
            client: Client::new(),
            input: OmegaInput::new(),
        }
    }

    pub fn save(&self, id: u64) {
        if !Path::new("./data").exists(){
            std::fs::create_dir("./data").unwrap();
        }
        let path = format!("./data/{}.json", id);
        let file = File::create(path).unwrap();
        serde_json::to_writer(file, &self.input).unwrap();
    }

    pub fn load(&mut self, id: u64) {
        let path = format!("./data/{}.json", id);
        if !Path::new(&path).exists() {
            return;
        }
        let file = File::open(path).unwrap();
        self.input = serde_json::from_reader(file).unwrap();
    }

    pub fn set_key(&mut self, key: &str) {
        self.api_key = key.to_string();
    }

    pub fn get_msg(&self) -> Vec<Message> {
        self.input.messages.clone()
    }

    pub fn push_msg(&mut self, role: Role, content: String) {
        self.input.push(role, content);
    }

    pub async fn chat(&mut self, req: &str)  -> Result<Pin<Box<dyn Stream<Item = Result<(String, bool), String>> + Send>>, String>  {
        self.push_msg(Role::User, req.to_string());

        let response = self.client
            .post("https://api.deepseek.com/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .json(&self.input)
            .send()
            .await
            .expect("Failed to send request");

        if !response.status().is_success() {
            return Err(format!("Failed to get a response: {}", response.status()));
        }

        let stream = response.bytes_stream()
        // 过滤掉最后一个无效 chunk
        .filter(|item| {
            future::ready(match item {
                Ok(chunk) => {
                    let json_str = String::from_utf8_lossy(&chunk);
                    !json_str.contains("data: [DONE]")
                }
                Err(_) => true,
            })
        })
        .map(move |item| {
            match item {
                Ok(chunk) => {
                    let json_str = String::from_utf8_lossy(&chunk);

                    // 有一定概率返回的数据是两个 Chunk 连在一起, 所以直接全部尝试拆分处理
                    let choice = json_str.split("data: ")
                        .map(|s| s.trim())
                        .filter(|s| !s.is_empty())
                        .map(|s| {
                            if let Ok(json_obj) = serde_json::from_str::<OmegaOutput>(s){
                                json_obj.get_choices()
                            } else {
                                (format!("[Failed parse: {}]", s), false)
                            }
                        })
                        .fold((String::new(), false), |(mut acc_s, mut acc_b), (s, b)| {
                            acc_s.push_str(&s);
                            acc_b |= b;
                            (acc_s, acc_b)
                        });
                    Ok(choice)
                }
                Err(e) => Err(format!("Error while reading stream: {}", e)),
            }
        })
        // 无论如何都让聊天完成
        .chain(futures::stream::once(future::ready(Ok((String::new(), true)))));

        Ok(Box::pin(stream))
    }
}


/// 模型输入
#[derive(Debug, Deserialize, Serialize)]
pub struct OmegaInput {
    messages: Vec<Message>,
    model: String,
    /// 介于 -2.0 和 2.0 之间的数字. 如果该值为正, 那么新 token 会根据其是否已在已有文本中出现受到相应的惩罚, 从而增加模型谈论新主题的可能性
    presence_penalty: f64,
    /// 介于 -2.0 和 2.0 之间的数字. 如果该值为正, 那么新 token 会根据其在已有文本中的出现频率受到相应的惩罚, 降低模型重复相同内容的可能性
    frequency_penalty: f64,
    max_tokens: u32,
    stream: bool,
    /// 采样温度，介于 0 和 2 之间. 更高的值, 如 0.8, 会使输出更随机, 而更低的值会使其更加集中和确定. 建议可以更改这个值或者更改 top_p, 不建议同时修改
    temperature: f64,
    /// 作为调节采样温度的替代方案, 模型会考虑前 top_p 概率的 token 的结果. 所以 0.1 就意味着只有包括在最高 10% 概率中的 token 会被考虑
    top_p: f64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
    pub name: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Role {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "system")]
    System,
    #[serde(rename = "assistant")]
    Assistant,
}

impl OmegaInput {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            model: "deepseek-chat".to_string(),
            frequency_penalty: 0.0,
            max_tokens: 2048,
            presence_penalty: 0.0,
            stream: true,
            temperature: 1.0,
            top_p: 1.0,
        }
    }

    pub fn push(&mut self, role: Role, content: String) {
        self.messages.push(Message::new(role, content));
    }
}

impl Message {
    pub fn new(role: Role, content: String) -> Self {
        Self {
            role,
            content,
            name: None,
        }
    }
}

/// 模型输出
#[derive(Deserialize, Debug)]
pub struct OmegaOutput {
    choices: Vec<Choice>,
}

#[derive(Deserialize, Debug)]
struct Choice {
    delta: Delta,
    finish_reason: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Delta {
    content: Option<String>,
}

impl OmegaOutput {
    pub fn get_choices(&self) -> (String, bool) {
        // 只获取第一个choice里面的content字符串和finish_reason字符串
        let choice = &self.choices[0];
        (
            choice.delta.content.to_owned().unwrap_or(String::new()),
            choice.finish_reason.as_deref().map_or(false, |f| {
                if f.is_empty() {
                    false
                } else {
                    true
                }
            })
        )
    }
}

#[test]
fn test_parse() {
    let str = r#"

    data: {
        "id": "",
        "choices": [
            {
                "delta": {
                    "content": "你",
                    "function_call": null,
                    "role": "assistant",
                    "tool_calls": null
                },
                "finish_reason": null,
                "index": 0,
                "logprobs": null
            }
        ],
        "created": 1721823365,
        "model": "qwen-vl-plus",
        "object": "chat.completion.chunk",
        "service_tier": null,
        "system_fingerprint": null,
        "usage": null
    }

    data: {
        "id": "",
        "choices": [
            {
                "delta": {
                    "content": "好",
                    "function_call": null,
                    "role": "assistant",
                    "tool_calls": null
                },
                "finish_reason": null,
                "index": 0,
                "logprobs": null
            }
        ],
        "created": 1721823365,
        "model": "qwen-vl-plus",
        "object": "chat.completion.chunk",
        "service_tier": null,
        "system_fingerprint": null,
        "usage": null
    }

    data: {
        "id": "",
        "choices": [
            {
                "delta": {
                    "content": "",
                    "function_call": null,
                    "role": "assistant",
                    "tool_calls": null
                },
                "finish_reason": "stop",
                "index": 0,
                "logprobs": null
            }
        ],
        "created": 1721823365,
        "model": "qwen-vl-plus",
        "object": "chat.completion.chunk",
        "service_tier": null,
        "system_fingerprint": null,
        "usage": null
    }

    "#;
    let (res_s, res_b) = str.split("data: ")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| {
            if let Ok(json_obj) = serde_json::from_str::<OmegaOutput>(s){
                json_obj.get_choices()
            } else {
                (format!("[Failed parse: {}]", s), false)
            }
        })
        .fold((String::new(), false), |(mut acc_s, mut acc_b), (s, b)| {
            acc_s.push_str(&s);
            println!("{}", b);
            acc_b |= b;
            (acc_s, acc_b)
        });
    let json_obj = (res_s, res_b);
    println!("{:?}", json_obj);
}

#[tokio::test]
async fn test_chat() {
    use futures::StreamExt;
    use std::io::Write;
    use crate::cfg::Cfg;

    let cfg = Cfg::load();

    let mut omega = Omega::new();

    omega.set_key(cfg.get_key());

    let mut res = omega.chat("你好").await.unwrap();

    while let Some(item) = res.next().await {
        match item {
            Ok((message, done)) => {
                std::io::stdout().write_all(message.as_bytes()).expect("Failed to write to stdout");
                std::io::stdout().flush().expect("Failed to flush stdout");
                if done {
                    println!("finsh");
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

#[test]
fn test_chat_history() {
    let mut omega = Omega::new();
    // omega.push_msg(Role::User, "你好呀".to_string());
    // omega.push_msg(Role::Assistant, "你好呀".to_string());
    // omega.push_msg(Role::User, "你好呀".to_string());
    // omega.push_msg(Role::Assistant, "你好呀".to_string());
    // omega.save(2);
    omega.load(1);
    println!("{:?}", omega.get_msg());
    omega.load(2);
    println!("{:?}", omega.get_msg());
}
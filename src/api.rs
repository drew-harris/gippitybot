use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Payload {
    model: String,
    n: u32,
    max_tokens: u32,
    temperature: f32,
    top_p: f32,
    messages: Vec<Message>,
}

pub fn get_message(_history: Vec<String>) -> Result<String, reqwest::Error> {
    let _firework_token = env::var("FIREWORK_TOKEN").expect("Get firework token");
    return Ok("test".to_string());
}

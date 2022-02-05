#![allow(dead_code)]
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct Request {
    pub prompt: String,
    pub max_tokens: usize,
    pub temperature: f32,
    pub n: u8,
}

#[derive(Deserialize, Debug)]
pub struct Response {
    id: Option<String>,
    object: Option<String>,
    created: Option<usize>,
    model: Option<String>,
    pub choices: Vec<Choice>,
}

#[derive(Deserialize, Debug)]
pub struct Choice {
    pub text: String,
    index: usize,
    logprobs: Option<isize>,
    finish_reason: String,
}

impl Default for Choice {
    fn default() -> Self {
        Choice {
            text: "...".to_string(),
            ..Default::default()
        }
    }
}

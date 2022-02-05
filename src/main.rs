#![allow(dead_code)]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct Request {
    prompt: Option<String>,
    max_tokens: Option<usize>,
    temperature: Option<f32>,
    top_p: Option<f32>,
    n: Option<usize>,
    stream: Option<bool>,
    logprobs: Option<usize>,
    echo: Option<bool>,
    stop: Option<Vec<String>>,
    presence_penalty: Option<f32>,
    frequency_penalty: Option<f32>,
    best_of: Option<usize>,
    logit_bias: Option<HashMap<String, isize>>,
}

impl Default for Request {
    fn default() -> Self {
        Request {
            prompt: None,
            max_tokens: Some(16),
            temperature: Some(1.0),
            top_p: Some(1.0),
            n: Some(1),
            stream: Some(false),
            logprobs: None,
            echo: Some(false),
            stop: None,
            presence_penalty: Some(0.0),
            frequency_penalty: Some(0.0),
            best_of: Some(1),
            logit_bias: Some(HashMap::new()),
        }
    }
}

#[derive(Deserialize, Debug)]
struct Response {
    id: String,
    object: String,
    created: usize,
    model: String,
    choices: Vec<Choice>,
}

#[derive(Deserialize, Debug)]
struct Choice {
    text: String,
    index: usize,
    logprobs: Option<isize>,
    finish_reason: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let prompt = {
        let mut args = std::env::args();
        args.next();
        args.next().unwrap_or_else(|| "".to_string())
    };

    let req = Request {
        prompt: Some(prompt),
        ..Default::default()
    };

    let client = reqwest::Client::new();
    let res = client
        .post("https://api.openai.com/v1/engines/text-davinci-001/completions")
        .bearer_auth(std::fs::read_to_string("api_key")?)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&req).unwrap())
        .send()
        .await?
        .text()
        .await?;

    let res: Response = serde_json::from_str(&res).unwrap();

    for choice in res.choices {
        println!("{}", choice.text);
    }

    Ok(())
}

#![allow(dead_code)]
use std::io::Write;

use serde::{Deserialize, Serialize};
use spinners::{utils::spinner_names, Spinner};

#[derive(Serialize, Debug)]
struct Request {
    prompt: String,
    max_tokens: usize,
    temperature: f32,
    n: u8,
}

#[derive(Deserialize, Debug)]
struct Response {
    id: Option<String>,
    object: Option<String>,
    created: Option<usize>,
    model: Option<String>,
    choices: Vec<Choice>,
}

#[derive(Deserialize, Debug)]
struct Choice {
    text: String,
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let url =
        "https://api.openai.com/v1/engines/text-davinci-001/completions";

    let client = reqwest::Client::new();
    let token = std::fs::read_to_string("./api_key")?;
    print!("{esc}c", esc = 27 as char);
    println!("OpenAI is ready. Ask it anything. Press Ctrl-C to quit.");
    loop {
        print!("> ");
        std::io::stdout().flush()?;
        let mut prompt = String::new();
        std::io::stdin().read_line(&mut prompt)?;
        std::io::stdout().flush()?;

        let spinner = Spinner::new(
            &spinner_names::SpinnerNames::Dots,
            "OpenAI is typing...".to_string(),
        );

        let req = Request {
            prompt,
            max_tokens: 64,
            n: 1,
            temperature: 0.8,
        };

        let res = client
            .post(url)
            .bearer_auth(&token)
            .header("Content-Type", "application/json")
            .json(&req)
            .send()
            .await?
            .text()
            .await?;

        spinner.stop();

        let res: Response = serde_json::from_str(&res)?;
        if let Some(choice) = res.choices.get(0) {
            println!("{}", choice.text)
        }
    }
}

mod structs;
use spinners::{utils::spinner_names, Spinner};
use std::io::Write;
use structs::{Request, Response};

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

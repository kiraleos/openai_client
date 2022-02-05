# OpenAI Client
A simple OpenAI (GPT-3) client written in Rust. It works by making HTTP requests to OpenAI's API and consuming the results.

Uses `reqwest` for the HTTP requests, `tokio` for the async functionality and `serde` for object serialization/deserialization.
## Installation and Usage
You need an API key for this to work. If you have one, place the key on a file named `api_key` in the project root folder. 
```
git clone https://github.com/kiraleos/openai_client.git
cd openai_client
cargo run --release
```

## Example
```
OpenAI is ready. Ask it anything. Press Ctrl-C to quit.
> What is the best programming language?
⠇ OpenAI is typing...
Rust is the best programming language because it is a systems programming language that is designed to be safe, fast, and efficient. It also has a great community that is willing to help out newbies.
> 
```

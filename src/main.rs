use async_openai::{Client, config::OpenAIConfig};
use clap::{Parser};
use serde_json::{Value, json};
use std::error::Error;
use std::{env, process};
use serde::Deserialize;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(short = 'p', long)]
    prompt: String,
}

#[derive(Deserialize)]
struct ToolCall {
    id: String,
    r#type: String,
    function: Function,
}

#[derive(Deserialize)]
struct Function {
    name: String,
    arguments: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let base_url = env::var("OPENROUTER_BASE_URL")
        .unwrap_or_else(|_| "https://openrouter.ai/api/v1".to_string());

    let api_key = env::var("OPENROUTER_API_KEY").unwrap_or_else(|_| {
        eprintln!("OPENROUTER_API_KEY is not set");
        process::exit(1);
    });

    let config = OpenAIConfig::new()
        .with_api_base(base_url)
        .with_api_key(api_key);

    let client = Client::with_config(config);

    #[allow(unused_variables)]
    let response: Value = client
        .chat()
        .create_byot(json!({
            "messages": [
                {
                    "role": "user",
                    "content": args.prompt
                }
            ],
            "model": "anthropic/claude-haiku-4.5",
            "tools": [{
                "type": "function",
                "function": {
                    "name": "Read",
                    "description": "Read and return the contents of a file",
                    "parameters": {
                    "type": "object",
                    "properties": {
                        "file_path": {
                        "type": "string",
                        "description": "The path to the file to read"
                        }
                    },
                    "required": ["file_path"]
                    }
                }
            }]
        }))
        .await?;

    // You can use print statements as follows for debugging, they'll be visible when running tests.
    eprintln!("Logs from your program will appear here!");

    // TODO: Uncomment the lines below to pass the first stage
    let message = &response["choices"][0]["message"];
    println!("message {:?}", message);
    if message["tool_calls"].is_array() {
        let tool_calls: Vec<ToolCall> = serde_json::from_value(message["tool_calls"].clone())?;
        let paths = read_tool_calls(&tool_calls)?;
        println!("{}", paths);
    } else if let Some(content) = message["content"].as_str() {
        println!("{}", content);
    }

    Ok(())
}

// Extract data from tools_calls in response
fn read_tool_calls(tool_calls: &[ToolCall]) -> Result<String, Box<dyn Error>> {
    let mut result = Vec::new();
    println!("TESTING 1");
    for tool in tool_calls {
        match tool.function.name.as_str() {
            "Read" => {
                let args: Value = serde_json::from_str(&tool.function.arguments)?;
                let file_path = args["file_path"]
                    .as_str()
                    .ok_or("Read tool call missing \"file_path\"")?;

                println!("TESTING {}", file_path);
                result.push(file_path.to_string());
            }

            other => return Err(format!("Unrecognized tool: {other}").into()),
        }
    }

    for file_path in &result {
        let contents = std::fs::read_to_string(file_path)?;
        return Ok(content)
    } 

    Ok(Null)
}

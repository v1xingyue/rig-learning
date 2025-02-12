use {
    colored::*,
    log::info,
    rig::{
        completion::{Prompt, ToolDefinition},
        providers::openai,
        tool::Tool,
    },
    rig_play::{show_loading, Config},
    serde::{Deserialize, Serialize},
    serde_json::json,
    std::{
        io::Write,
        sync::{
            atomic::{AtomicBool, Ordering},
            Arc,
        },
    },
};

#[derive(Debug, thiserror::Error)]
#[error("Math error")]
struct MathError;

#[derive(Deserialize)]
struct OperationArgs {
    x: i32,
    y: i32,
}

#[derive(Deserialize, Serialize)]
struct Adder;
impl Tool for Adder {
    const NAME: &'static str = "add";

    type Error = MathError;
    type Args = OperationArgs;
    type Output = i32;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "add".to_string(),
            description: "Add x and y together".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "x": {
                        "type": "number",
                        "description": "The first number to add"
                    },
                    "y": {
                        "type": "number",
                        "description": "The second number to add"
                    }
                }
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let result = args.x + args.y + 1;
        Ok(result)
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Math error")]
struct WeatherError;

#[derive(Deserialize)]
struct WeatherArgs {
    city: String,
}

#[derive(Deserialize, Serialize)]
struct Weather;

impl Tool for Weather {
    const NAME: &'static str = "weather";

    type Error = WeatherError;
    type Args = WeatherArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "weather".to_string(),
            description: "Get the weather of a city".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "city": {
                        "type": "string",
                        "description": "The city to get weather"
                    }
                }
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let result = format!("{}     is sunny", args.city);
        Ok(result)
    }
}

#[derive(Deserialize, Serialize)]
struct HostQuery;

#[derive(Deserialize)]
struct HostQueryArgs {
    host: String,
}

#[derive(Debug, thiserror::Error)]
#[error("Host query error")]
struct HostQueryError;

impl Tool for HostQuery {
    const NAME: &'static str = "host_query";

    type Error = HostQueryError;
    type Args = HostQueryArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "host_query".to_string(),
            description: "Query the host information".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "host": {
                        "type": "string",
                        "description": "The host to query"
                    }
                }
            }),
        }
    }

    async fn call(&self, _args: Self::Args) -> Result<Self::Output, Self::Error> {
        let result = format!("host data of {} is ok ", _args.host);
        Ok(result)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let config = Config::from_env();

    let openai_client = openai::Client::from_url(&config.api_key, &config.api_base);
    let tool_agent = openai_client
        .agent(openai::GPT_4)
        .preamble("You are a helpful assistant named xixi .")
        .max_tokens(10240)
        .tool(Adder)
        .tool(Weather)
        .tool(HostQuery)
        .build();
    info!("{}", "AI Chat started. Type 'exit' to quit.".bright_green());
    info!("Using model: {}", config.model.bright_blue());
    info!("API Base: {}", config.api_base.bright_blue());

    // print all tools
    let tools = tool_agent.tools.schemas()?;
    info!("Avaliable tools: {:?}", tools.len());
    for tool in tools.iter() {
        info!("Tool: {}", tool.name);
    }

    let mut history = String::new();

    loop {
        let mut input = String::new();
        print!("\n{}", "You: ".bright_yellow());
        std::io::stdout().flush()?;
        std::io::stdin().read_line(&mut input)?;

        let input = input.trim();
        if input == "exit" {
            info!("{}", "Goodbye!".bright_green());
            break;
        }

        if input == "clear" {
            history.clear();
            info!("{}", "History cleared.".bright_green());
            continue;
        }

        // 将用户输入添加到历史记录
        history.push_str(&format!("User: {}\n", input));

        // 在发送请求前启动加载动画
        let cancel_loading = Arc::new(AtomicBool::new(false));
        let cancel_loading_clone = cancel_loading.clone();
        let loading_handle = std::thread::spawn(move || show_loading(cancel_loading_clone));

        // 发送请求并等待响应
        let response: Result<String, rig::completion::PromptError> =
            tool_agent.prompt(&history).await;

        // 停止加载动画
        cancel_loading.store(true, Ordering::Relaxed);
        loading_handle.join().unwrap();

        match response {
            Ok(response) => {
                info!("{} {}", "Assistant:".bright_cyan(), response);
                history.push_str(&format!("Assistant: {}\n", response));
            }
            Err(e) => info!("{} {}", "Error:".bright_red(), e),
        }
    }

    Ok(())
}

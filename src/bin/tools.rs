use {
    colored::*,
    log::info,
    rig::{
        completion::{Chat, Message},
        providers::openai,
    },
    rig_play::{show_loading, tools::*, Config},
    std::{
        io::Write,
        sync::{
            atomic::{AtomicBool, Ordering},
            Arc,
        },
    },
};

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
    let documents = &tool_agent.tools.documents().await?;
    info!("tools added: {}", documents.len());
    for document in documents.iter() {
        info!("Tool: {}", document.id);
    }

    let mut history = vec![];

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
        history.push(Message {
            role: "user".to_string(),
            content: input.to_string(),
        });

        // 在发送请求前启动加载动画
        let cancel_loading = Arc::new(AtomicBool::new(false));
        let cancel_loading_clone = cancel_loading.clone();
        let loading_handle: std::thread::JoinHandle<()> =
            std::thread::spawn(move || show_loading(cancel_loading_clone));

        // 发送请求并等待响应
        let response: Result<String, rig::completion::PromptError> =
            tool_agent.chat(&input, history.clone()).await;

        // 停止加载动画
        cancel_loading.store(true, Ordering::Relaxed);
        loading_handle.join().unwrap();

        match response {
            Ok(response) => {
                info!("{} {}", "Assistant:".bright_cyan(), response);
                history.push(Message {
                    role: "assistant".to_string(),
                    content: response.to_string(),
                });
            }
            Err(e) => info!("{} {}", "Error:".bright_red(), e),
        }
    }

    Ok(())
}

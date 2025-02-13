use {
    colored::*,
    log::info,
    rig::{
        completion::{Chat, Message},
        providers::openai,
    },
    rig_play::{show_loading, Config},
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
    let chat_model = openai_client.agent(config.model.as_str()).build();
    println!("{}", "AI Chat started. Type 'exit' to quit.".bright_green());
    println!("Using model: {}", config.model.bright_blue());
    println!("API Base: {}", config.api_base.bright_blue());

    let mut history = vec![];

    loop {
        let mut input = String::new();
        print!("\n{}", "You: ".bright_yellow());
        std::io::stdout().flush()?;
        std::io::stdin().read_line(&mut input)?;

        let input = input.trim();
        if input == "exit" {
            println!("{}", "Goodbye!".bright_green());
            break;
        }

        if input == "clear" {
            history.clear();
            println!("{}", "History cleared.".bright_green());
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
        let loading_handle = std::thread::spawn(move || show_loading(cancel_loading_clone));

        info!("Sending request to chat model");
        // 发送请求并等待响应
        let response = chat_model.chat(input, history.clone()).await;

        // 停止加载动画
        cancel_loading.store(true, Ordering::Relaxed);
        loading_handle.join().unwrap();

        match response {
            Ok(response) => {
                println!(
                    "{} {}",
                    "Assistant:".bright_cyan(),
                    response.strip_prefix("Assistant: ").unwrap_or(&response)
                );
                history.push(Message {
                    role: "assistant".to_string(),
                    content: response,
                });
            }
            Err(e) => println!("{} {}", "Error:".bright_red(), e),
        }
    }
    Ok(())
}

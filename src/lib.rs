use {
    colored::*,
    log::info,
    std::io::Write,
    std::sync::atomic::{AtomicBool, Ordering},
    std::sync::Arc,
    std::time::Duration,
};

// 配置结构
pub struct Config {
    pub api_key: String,
    pub api_base: String,
    pub model: String,
}

impl Config {
    pub fn new(api_key: String, api_base: String, model: String) -> Self {
        Self {
            api_key,
            api_base,
            model,
        }
    }

    pub fn from_env() -> Self {
        env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
        info!(
            "{}",
            "Loading config from environment variables".bright_blue()
        );
        Self::new(
            std::env::var("OPENAI_API_KEY").unwrap(),
            std::env::var("OPENAI_API_BASE").unwrap(),
            std::env::var("OPENAI_MODEL").unwrap(),
        )
    }
}

// 修改加载动画函数
pub fn show_loading(cancel: Arc<AtomicBool>) {
    let spinner = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    let mut i = 0;
    print!("\r");
    while !cancel.load(Ordering::Relaxed) {
        print!(
            "\r{} {}",
            "Thinking".bright_blue(),
            spinner[i].bright_blue()
        );
        std::io::stdout().flush().unwrap();
        std::thread::sleep(Duration::from_millis(100));
        i = (i + 1) % spinner.len();
    }
    print!("\r"); // 清除加载动画
}

use std::path::PathBuf;

use dirs::home_dir;
use log::{info, LevelFilter};
use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
use log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller;
use log4rs::append::rolling_file::policy::compound::trigger::size::SizeTrigger;
use log4rs::append::rolling_file::RollingFileAppender;
use log4rs::Config;
use log4rs::config::{Appender, Root};
use tower_lsp::{LspService, Server};

use java_language_server::lsp::backend::Backend;

#[tokio::main]
async fn main() {
    setup_logging();

    info!("Init LSP");

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::build(|client| Backend {
        client,
    })
        .finish();

    Server::new(stdin, stdout, socket).serve(service).await;
}

fn setup_logging() {

    let trigger_size = 30 * 1024 * 1024;
    let trigger = Box::new(SizeTrigger::new(trigger_size));

    let roller_pattern = "step/step_{}.gz";
    let roller_count = 5;
    let roller_base = 1;
    let roller = Box::new(
        FixedWindowRoller::builder()
            .base(roller_base)
            .build(roller_pattern, roller_count)
            .unwrap(),
    );

    let compound_policy = Box::new(CompoundPolicy::new(trigger, roller));

    let file_appender = RollingFileAppender::builder()
        .build(get_logging_path().join("general.log"), compound_policy)
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("log_file", Box::new(file_appender)))
        .build(Root::builder().appender("log_file").build(LevelFilter::Info))
        .unwrap();

    // You can use handle to change logger config at runtime
    log4rs::init_config(config).unwrap();
}

fn get_logging_path() -> PathBuf {
    home_dir().unwrap().join("Library/Logs/Java-LSP")
}
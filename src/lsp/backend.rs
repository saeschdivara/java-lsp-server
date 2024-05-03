use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;

use log::info;
use serde_json::Value;
use tower_lsp::{Client, LanguageServer};
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;

#[derive(Debug)]
pub struct Backend {
    pub client: Client,
    pub root_path: PathBuf,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        info!("Init backend");

        Ok(InitializeResult {
            capabilities: Default::default(),
            server_info: None,
            offset_encoding: None,
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        info!("LSP > initialized");

        self.client
            .log_message(MessageType::INFO, "initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        info!("LSP > shutdown");

        Ok(())
    }

}
use std::future::Future;
use std::pin::Pin;

use log::info;
use serde_json::Value;
use tower_lsp::{Client, LanguageServer};
use tower_lsp::lsp_types::{CallHierarchyIncomingCall, CallHierarchyIncomingCallsParams, CallHierarchyItem, CallHierarchyOutgoingCall, CallHierarchyOutgoingCallsParams, CallHierarchyPrepareParams, CodeAction, CodeActionParams, CodeActionResponse, CodeLens, CodeLensParams, ColorInformation, ColorPresentation, ColorPresentationParams, CompletionItem, CompletionParams, CompletionResponse, CreateFilesParams, DeleteFilesParams, DidChangeConfigurationParams, DidChangeTextDocumentParams, DidChangeWatchedFilesParams, DidChangeWorkspaceFoldersParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams, DidSaveTextDocumentParams, DocumentColorParams, DocumentDiagnosticParams, DocumentDiagnosticReportResult, DocumentFormattingParams, DocumentHighlight, DocumentHighlightParams, DocumentLink, DocumentLinkParams, DocumentOnTypeFormattingParams, DocumentRangeFormattingParams, DocumentSymbolParams, DocumentSymbolResponse, ExecuteCommandParams, FoldingRange, FoldingRangeParams, GotoDefinitionParams, GotoDefinitionResponse, Hover, HoverParams, InitializedParams, InitializeParams, InitializeResult, InlayHint, InlayHintParams, InlineValue, InlineValueParams, LinkedEditingRangeParams, LinkedEditingRanges, Location, Moniker, MonikerParams, PrepareRenameResponse, ReferenceParams, RenameFilesParams, RenameParams, SelectionRange, SelectionRangeParams, SemanticTokensDeltaParams, SemanticTokensFullDeltaResult, SemanticTokensParams, SemanticTokensRangeParams, SemanticTokensRangeResult, SemanticTokensResult, SignatureHelp, SignatureHelpParams, SymbolInformation, TextDocumentPositionParams, TextEdit, TypeHierarchyItem, TypeHierarchyPrepareParams, TypeHierarchySubtypesParams, TypeHierarchySupertypesParams, WillSaveTextDocumentParams, WorkspaceDiagnosticParams, WorkspaceDiagnosticReportResult, WorkspaceEdit, WorkspaceSymbol, WorkspaceSymbolParams};
use tower_lsp::lsp_types::request::{GotoDeclarationParams, GotoDeclarationResponse, GotoImplementationParams, GotoImplementationResponse, GotoTypeDefinitionParams, GotoTypeDefinitionResponse};

#[derive(Debug)]
pub struct Backend {
    pub client: Client,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> tower_lsp::jsonrpc::Result<InitializeResult> {
        info!("Init backend");

        Ok(InitializeResult {
            capabilities: Default::default(),
            server_info: None,
            offset_encoding: None,
        })
    }

    fn initialized<'life0, 'async_trait>(&'life0 self, params: InitializedParams) -> Pin<Box<dyn Future<Output=()> + Send + 'async_trait>> where 'life0: 'async_trait, Self: 'async_trait {
        info!("LSP > initialized");
        todo!()
    }

    fn shutdown<'life0, 'async_trait>(&'life0 self) -> Pin<Box<dyn Future<Output=tower_lsp::jsonrpc::Result<()>> + Send + 'async_trait>> where 'life0: 'async_trait, Self: 'async_trait {
        info!("LSP > shutdown");
        todo!()
    }

    fn did_open<'life0, 'async_trait>(&'life0 self, params: DidOpenTextDocumentParams) -> Pin<Box<dyn Future<Output=()> + Send + 'async_trait>> where 'life0: 'async_trait, Self: 'async_trait {
        info!("LSP > did_open");
        todo!()
    }

    fn did_change<'life0, 'async_trait>(&'life0 self, params: DidChangeTextDocumentParams) -> Pin<Box<dyn Future<Output=()> + Send + 'async_trait>> where 'life0: 'async_trait, Self: 'async_trait {
        info!("LSP > did_change");
        todo!()
    }

    fn will_save<'life0, 'async_trait>(&'life0 self, params: WillSaveTextDocumentParams) -> Pin<Box<dyn Future<Output=()> + Send + 'async_trait>> where 'life0: 'async_trait, Self: 'async_trait {
        info!("LSP > will_save");
        todo!()
    }

    fn will_save_wait_until<'life0, 'async_trait>(&'life0 self, params: WillSaveTextDocumentParams) -> Pin<Box<dyn Future<Output=tower_lsp::jsonrpc::Result<Option<Vec<TextEdit>>>> + Send + 'async_trait>> where 'life0: 'async_trait, Self: 'async_trait {
        info!("LSP > will_save_wait_until");
        todo!()
    }

    fn did_save<'life0, 'async_trait>(&'life0 self, params: DidSaveTextDocumentParams) -> Pin<Box<dyn Future<Output=()> + Send + 'async_trait>> where 'life0: 'async_trait, Self: 'async_trait {
        info!("LSP > did_save");
        todo!()
    }

    fn did_close<'life0, 'async_trait>(&'life0 self, params: DidCloseTextDocumentParams) -> Pin<Box<dyn Future<Output=()> + Send + 'async_trait>> where 'life0: 'async_trait, Self: 'async_trait {
        info!("LSP > did_close");
        todo!()
    }

    fn goto_declaration<'life0, 'async_trait>(&'life0 self, params: GotoDeclarationParams) -> Pin<Box<dyn Future<Output=tower_lsp::jsonrpc::Result<Option<GotoDeclarationResponse>>> + Send + 'async_trait>> where 'life0: 'async_trait, Self: 'async_trait {
        info!("LSP > goto_declaration");
        todo!()
    }

    fn goto_definition<'life0, 'async_trait>(&'life0 self, params: GotoDefinitionParams) -> Pin<Box<dyn Future<Output=tower_lsp::jsonrpc::Result<Option<GotoDefinitionResponse>>> + Send + 'async_trait>> where 'life0: 'async_trait, Self: 'async_trait {
        info!("LSP > goto_definition");
        todo!()
    }

    fn goto_type_definition<'life0, 'async_trait>(&'life0 self, params: GotoTypeDefinitionParams) -> Pin<Box<dyn Future<Output=tower_lsp::jsonrpc::Result<Option<GotoTypeDefinitionResponse>>> + Send + 'async_trait>> where 'life0: 'async_trait, Self: 'async_trait {
        info!("LSP > goto_type_definition");
        todo!()
    }

    fn goto_implementation<'life0, 'async_trait>(&'life0 self, params: GotoImplementationParams) -> Pin<Box<dyn Future<Output=tower_lsp::jsonrpc::Result<Option<GotoImplementationResponse>>> + Send + 'async_trait>> where 'life0: 'async_trait, Self: 'async_trait {
        info!("LSP > goto_implementation");
        todo!()
    }

    fn references<'life0, 'async_trait>(&'life0 self, params: ReferenceParams) -> Pin<Box<dyn Future<Output=tower_lsp::jsonrpc::Result<Option<Vec<Location>>>> + Send + 'async_trait>> where 'life0: 'async_trait, Self: 'async_trait {
        info!("LSP > references");
        todo!()
    }

    fn prepare_call_hierarchy<'life0, 'async_trait>(&'life0 self, params: CallHierarchyPrepareParams) -> Pin<Box<dyn Future<Output=tower_lsp::jsonrpc::Result<Option<Vec<CallHierarchyItem>>>> + Send + 'async_trait>> where 'life0: 'async_trait, Self: 'async_trait {
        info!("LSP > prepare_call_hierarchy");
        todo!()
    }

    fn incoming_calls<'life0, 'async_trait>(&'life0 self, params: CallHierarchyIncomingCallsParams) -> Pin<Box<dyn Future<Output=tower_lsp::jsonrpc::Result<Option<Vec<CallHierarchyIncomingCall>>>> + Send + 'async_trait>> where 'life0: 'async_trait, Self: 'async_trait {
        info!("LSP > incoming_calls");
        todo!()
    }

    fn outgoing_calls<'life0, 'async_trait>(&'life0 self, params: CallHierarchyOutgoingCallsParams) -> Pin<Box<dyn Future<Output=tower_lsp::jsonrpc::Result<Option<Vec<CallHierarchyOutgoingCall>>>> + Send + 'async_trait>> where 'life0: 'async_trait, Self: 'async_trait {
        info!("LSP > outgoing_calls");
        todo!()
    }

    fn prepare_type_hierarchy<'life0, 'async_trait>(&'life0 self, params: TypeHierarchyPrepareParams) -> Pin<Box<dyn Future<Output=tower_lsp::jsonrpc::Result<Option<Vec<TypeHierarchyItem>>>> + Send + 'async_trait>> where 'life0: 'async_trait, Self: 'async_trait {
        info!("LSP > prepare_type_hierarchy");
        todo!()
    }

    fn supertypes<'life0, 'async_trait>(&'life0 self, params: TypeHierarchySupertypesParams) -> Pin<Box<dyn Future<Output=tower_lsp::jsonrpc::Result<Option<Vec<TypeHierarchyItem>>>> + Send + 'async_trait>> where 'life0: 'async_trait, Self: 'async_trait {
        info!("LSP > supertypes");
        todo!()
    }

    fn subtypes<'life0, 'async_trait>(&'life0 self, params: TypeHierarchySubtypesParams) -> Pin<Box<dyn Future<Output=tower_lsp::jsonrpc::Result<Option<Vec<TypeHierarchyItem>>>> + Send + 'async_trait>> where 'life0: 'async_trait, Self: 'async_trait {
        info!("LSP > subtypes");
        todo!()
    }

    fn document_highlight<'life0, 'async_trait>(&'life0 self, params: DocumentHighlightParams) -> Pin<Box<dyn Future<Output=tower_lsp::jsonrpc::Result<Option<Vec<DocumentHighlight>>>> + Send + 'async_trait>> where 'life0: 'async_trait, Self: 'async_trait {
        info!("LSP > document_highlight");
        todo!()
    }

    fn document_link<'life0, 'async_trait>(&'life0 self, params: DocumentLinkParams) -> Pin<Box<dyn Future<Output=tower_lsp::jsonrpc::Result<Option<Vec<DocumentLink>>>> + Send + 'async_trait>> where 'life0: 'async_trait, Self: 'async_trait {
        info!("LSP > document_link");
        todo!()
    }

    fn document_link_resolve<'life0, 'async_trait>(&'life0 self, params: DocumentLink) -> Pin<Box<dyn Future<Output=tower_lsp::jsonrpc::Result<DocumentLink>> + Send + 'async_trait>> where 'life0: 'async_trait, Self: 'async_trait {
        info!("LSP > document_link_resolve");
        todo!()
    }

    fn hover<'life0, 'async_trait>(&'life0 self, params: HoverParams) -> Pin<Box<dyn Future<Output=tower_lsp::jsonrpc::Result<Option<Hover>>> + Send + 'async_trait>> where 'life0: 'async_trait, Self: 'async_trait {
        info!("LSP > hover");
        todo!()
    }

    fn code_lens<'life0, 'async_trait>(&'life0 self, params: CodeLensParams) -> Pin<Box<dyn Future<Output=tower_lsp::jsonrpc::Result<Option<Vec<CodeLens>>>> + Send + 'async_trait>> where 'life0: 'async_trait, Self: 'async_trait {
        info!("LSP > code_lens");
        todo!()
    }

    fn code_lens_resolve<'life0, 'async_trait>(&'life0 self, params: CodeLens) -> Pin<Box<dyn Future<Output=tower_lsp::jsonrpc::Result<CodeLens>> + Send + 'async_trait>> where 'life0: 'async_trait, Self: 'async_trait {
        info!("LSP > code_lens_resolve");
        todo!()
    }

    fn folding_range<'life0, 'async_trait>(&'life0 self, params: FoldingRangeParams) -> Pin<Box<dyn Future<Output=tower_lsp::jsonrpc::Result<Option<Vec<FoldingRange>>>> + Send + 'async_trait>> where 'life0: 'async_trait, Self: 'async_trait {
        info!("LSP > folding_range");
        todo!()
    }

    fn selection_range<'life0, 'async_trait>(&'life0 self, params: SelectionRangeParams) -> Pin<Box<dyn Future<Output=tower_lsp::jsonrpc::Result<Option<Vec<SelectionRange>>>> + Send + 'async_trait>> where 'life0: 'async_trait, Self: 'async_trait {
        info!("LSP > selection_range");
        todo!()
    }

    fn document_symbol<'life0, 'async_trait>(&'life0 self, params: DocumentSymbolParams) -> Pin<Box<dyn Future<Output=tower_lsp::jsonrpc::Result<Option<DocumentSymbolResponse>>> + Send + 'async_trait>> where 'life0: 'async_trait, Self: 'async_trait {
        info!("LSP > document_symbol");
        todo!()
    }

    fn semantic_tokens_full<'life0, 'async_trait>(&'life0 self, params: SemanticTokensParams) -> Pin<Box<dyn Future<Output=tower_lsp::jsonrpc::Result<Option<SemanticTokensResult>>> + Send + 'async_trait>> where 'life0: 'async_trait, Self: 'async_trait {
        info!("LSP > semantic_tokens_full");
        todo!()
    }

    fn semantic_tokens_full_delta<'life0, 'async_trait>(&'life0 self, params: SemanticTokensDeltaParams) -> Pin<Box<dyn Future<Output=tower_lsp::jsonrpc::Result<Option<SemanticTokensFullDeltaResult>>> + Send + 'async_trait>> where 'life0: 'async_trait, Self: 'async_trait {
        info!("LSP > semantic_tokens_full_delta");
        todo!()
    }

    fn semantic_tokens_range<'life0, 'async_trait>(&'life0 self, params: SemanticTokensRangeParams) -> Pin<Box<dyn Future<Output=tower_lsp::jsonrpc::Result<Option<SemanticTokensRangeResult>>> + Send + 'async_trait>> where 'life0: 'async_trait, Self: 'async_trait {
        info!("LSP > semantic_tokens_range");
        todo!()
    }

    fn inline_value<'life0, 'async_trait>(&'life0 self, params: InlineValueParams) -> Pin<Box<dyn Future<Output=tower_lsp::jsonrpc::Result<Option<Vec<InlineValue>>>> + Send + 'async_trait>> where 'life0: 'async_trait, Self: 'async_trait {
        info!("LSP > inline_value");
        todo!()
    }

    fn inlay_hint<'life0, 'async_trait>(&'life0 self, params: InlayHintParams) -> Pin<Box<dyn Future<Output=tower_lsp::jsonrpc::Result<Option<Vec<InlayHint>>>> + Send + 'async_trait>> where 'life0: 'async_trait, Self: 'async_trait {
        info!("LSP > inlay_hint");
        todo!()
    }

    fn inlay_hint_resolve<'life0, 'async_trait>(&'life0 self, params: InlayHint) -> Pin<Box<dyn Future<Output=tower_lsp::jsonrpc::Result<InlayHint>> + Send + 'async_trait>> where 'life0: 'async_trait, Self: 'async_trait {
        info!("LSP > inlay_hint_resolve");
        todo!()
    }

    fn moniker<'life0, 'async_trait>(&'life0 self, params: MonikerParams) -> Pin<Box<dyn Future<Output=tower_lsp::jsonrpc::Result<Option<Vec<Moniker>>>> + Send + 'async_trait>> where 'life0: 'async_trait, Self: 'async_trait {
        info!("LSP > moniker");
        todo!()
    }

    fn completion<'life0, 'async_trait>(&'life0 self, params: CompletionParams) -> Pin<Box<dyn Future<Output=tower_lsp::jsonrpc::Result<Option<CompletionResponse>>> + Send + 'async_trait>> where 'life0: 'async_trait, Self: 'async_trait {
        info!("LSP > completion");
        todo!()
    }

    fn completion_resolve<'life0, 'async_trait>(&'life0 self, params: CompletionItem) -> Pin<Box<dyn Future<Output=tower_lsp::jsonrpc::Result<CompletionItem>> + Send + 'async_trait>> where 'life0: 'async_trait, Self: 'async_trait {
        info!("LSP > completion_resolve");
        todo!()
    }

    fn diagnostic<'life0, 'async_trait>(&'life0 self, params: DocumentDiagnosticParams) -> Pin<Box<dyn Future<Output=tower_lsp::jsonrpc::Result<DocumentDiagnosticReportResult>> + Send + 'async_trait>> where 'life0: 'async_trait, Self: 'async_trait {
        info!("LSP > diagnostic");
        todo!()
    }

    fn workspace_diagnostic<'life0, 'async_trait>(&'life0 self, params: WorkspaceDiagnosticParams) -> Pin<Box<dyn Future<Output=tower_lsp::jsonrpc::Result<WorkspaceDiagnosticReportResult>> + Send + 'async_trait>> where 'life0: 'async_trait, Self: 'async_trait {
        info!("LSP > workspace_diagnostic");
        todo!()
    }

    fn signature_help<'life0, 'async_trait>(&'life0 self, params: SignatureHelpParams) -> Pin<Box<dyn Future<Output=tower_lsp::jsonrpc::Result<Option<SignatureHelp>>> + Send + 'async_trait>> where 'life0: 'async_trait, Self: 'async_trait {
        info!("LSP > signature_help");
        todo!()
    }

    fn code_action<'life0, 'async_trait>(&'life0 self, params: CodeActionParams) -> Pin<Box<dyn Future<Output=tower_lsp::jsonrpc::Result<Option<CodeActionResponse>>> + Send + 'async_trait>> where 'life0: 'async_trait, Self: 'async_trait {
        info!("LSP > code_action");
        todo!()
    }

    fn code_action_resolve<'life0, 'async_trait>(&'life0 self, params: CodeAction) -> Pin<Box<dyn Future<Output=tower_lsp::jsonrpc::Result<CodeAction>> + Send + 'async_trait>> where 'life0: 'async_trait, Self: 'async_trait {
        info!("LSP > code_action_resolve");
        todo!()
    }

    fn document_color<'life0, 'async_trait>(&'life0 self, params: DocumentColorParams) -> Pin<Box<dyn Future<Output=tower_lsp::jsonrpc::Result<Vec<ColorInformation>>> + Send + 'async_trait>> where 'life0: 'async_trait, Self: 'async_trait {
        info!("LSP > document_color");
        todo!()
    }

    fn color_presentation<'life0, 'async_trait>(&'life0 self, params: ColorPresentationParams) -> Pin<Box<dyn Future<Output=tower_lsp::jsonrpc::Result<Vec<ColorPresentation>>> + Send + 'async_trait>> where 'life0: 'async_trait, Self: 'async_trait {
        info!("LSP > color_presentation");
        todo!()
    }

    fn formatting<'life0, 'async_trait>(&'life0 self, params: DocumentFormattingParams) -> Pin<Box<dyn Future<Output=tower_lsp::jsonrpc::Result<Option<Vec<TextEdit>>>> + Send + 'async_trait>> where 'life0: 'async_trait, Self: 'async_trait {
        info!("LSP > formatting");
        todo!()
    }

    fn range_formatting<'life0, 'async_trait>(&'life0 self, params: DocumentRangeFormattingParams) -> Pin<Box<dyn Future<Output=tower_lsp::jsonrpc::Result<Option<Vec<TextEdit>>>> + Send + 'async_trait>> where 'life0: 'async_trait, Self: 'async_trait {
        info!("LSP > range_formatting");
        todo!()
    }

    fn on_type_formatting<'life0, 'async_trait>(&'life0 self, params: DocumentOnTypeFormattingParams) -> Pin<Box<dyn Future<Output=tower_lsp::jsonrpc::Result<Option<Vec<TextEdit>>>> + Send + 'async_trait>> where 'life0: 'async_trait, Self: 'async_trait {
        info!("LSP > on_type_formatting");
        todo!()
    }

    fn rename<'life0, 'async_trait>(&'life0 self, params: RenameParams) -> Pin<Box<dyn Future<Output=tower_lsp::jsonrpc::Result<Option<WorkspaceEdit>>> + Send + 'async_trait>> where 'life0: 'async_trait, Self: 'async_trait {
        info!("LSP > rename");
        todo!()
    }

    fn prepare_rename<'life0, 'async_trait>(&'life0 self, params: TextDocumentPositionParams) -> Pin<Box<dyn Future<Output=tower_lsp::jsonrpc::Result<Option<PrepareRenameResponse>>> + Send + 'async_trait>> where 'life0: 'async_trait, Self: 'async_trait {
        info!("LSP > prepare_rename");
        todo!()
    }

    fn linked_editing_range<'life0, 'async_trait>(&'life0 self, params: LinkedEditingRangeParams) -> Pin<Box<dyn Future<Output=tower_lsp::jsonrpc::Result<Option<LinkedEditingRanges>>> + Send + 'async_trait>> where 'life0: 'async_trait, Self: 'async_trait {
        info!("LSP > linked_editing_range");
        todo!()
    }

    fn symbol<'life0, 'async_trait>(&'life0 self, params: WorkspaceSymbolParams) -> Pin<Box<dyn Future<Output=tower_lsp::jsonrpc::Result<Option<Vec<SymbolInformation>>>> + Send + 'async_trait>> where 'life0: 'async_trait, Self: 'async_trait {
        info!("LSP > symbol");
        todo!()
    }

    fn symbol_resolve<'life0, 'async_trait>(&'life0 self, params: WorkspaceSymbol) -> Pin<Box<dyn Future<Output=tower_lsp::jsonrpc::Result<WorkspaceSymbol>> + Send + 'async_trait>> where 'life0: 'async_trait, Self: 'async_trait {
        info!("LSP > symbol_resolve");
        todo!()
    }

    fn did_change_configuration<'life0, 'async_trait>(&'life0 self, params: DidChangeConfigurationParams) -> Pin<Box<dyn Future<Output=()> + Send + 'async_trait>> where 'life0: 'async_trait, Self: 'async_trait {
        info!("LSP > did_change_configuration");
        todo!()
    }

    fn did_change_workspace_folders<'life0, 'async_trait>(&'life0 self, params: DidChangeWorkspaceFoldersParams) -> Pin<Box<dyn Future<Output=()> + Send + 'async_trait>> where 'life0: 'async_trait, Self: 'async_trait {
        info!("LSP > did_change_workspace_folders");
        todo!()
    }

    fn will_create_files<'life0, 'async_trait>(&'life0 self, params: CreateFilesParams) -> Pin<Box<dyn Future<Output=tower_lsp::jsonrpc::Result<Option<WorkspaceEdit>>> + Send + 'async_trait>> where 'life0: 'async_trait, Self: 'async_trait {
        info!("LSP > will_create_files");
        todo!()
    }

    fn did_create_files<'life0, 'async_trait>(&'life0 self, params: CreateFilesParams) -> Pin<Box<dyn Future<Output=()> + Send + 'async_trait>> where 'life0: 'async_trait, Self: 'async_trait {
        info!("LSP > did_create_files");
        todo!()
    }

    fn will_rename_files<'life0, 'async_trait>(&'life0 self, params: RenameFilesParams) -> Pin<Box<dyn Future<Output=tower_lsp::jsonrpc::Result<Option<WorkspaceEdit>>> + Send + 'async_trait>> where 'life0: 'async_trait, Self: 'async_trait {
        info!("LSP > will_rename_files");
        todo!()
    }

    fn did_rename_files<'life0, 'async_trait>(&'life0 self, params: RenameFilesParams) -> Pin<Box<dyn Future<Output=()> + Send + 'async_trait>> where 'life0: 'async_trait, Self: 'async_trait {
        info!("LSP > did_rename_files");
        todo!()
    }

    fn will_delete_files<'life0, 'async_trait>(&'life0 self, params: DeleteFilesParams) -> Pin<Box<dyn Future<Output=tower_lsp::jsonrpc::Result<Option<WorkspaceEdit>>> + Send + 'async_trait>> where 'life0: 'async_trait, Self: 'async_trait {
        info!("LSP > will_delete_files");
        todo!()
    }

    fn did_delete_files<'life0, 'async_trait>(&'life0 self, params: DeleteFilesParams) -> Pin<Box<dyn Future<Output=()> + Send + 'async_trait>> where 'life0: 'async_trait, Self: 'async_trait {
        info!("LSP > did_delete_files");
        todo!()
    }

    fn did_change_watched_files<'life0, 'async_trait>(&'life0 self, params: DidChangeWatchedFilesParams) -> Pin<Box<dyn Future<Output=()> + Send + 'async_trait>> where 'life0: 'async_trait, Self: 'async_trait {
        info!("LSP > did_change_watched_files");
        todo!()
    }

    fn execute_command<'life0, 'async_trait>(&'life0 self, params: ExecuteCommandParams) -> Pin<Box<dyn Future<Output=tower_lsp::jsonrpc::Result<Option<Value>>> + Send + 'async_trait>> where 'life0: 'async_trait, Self: 'async_trait {
        info!("LSP > execute_command");
        todo!()
    }
}
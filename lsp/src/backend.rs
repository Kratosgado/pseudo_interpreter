use std::collections::HashMap;

use pseudo_interpreter::{
    Lexer, Parser,
    constants::{error_handler::PseudoError, keywords::KEYWORDS},
};
use tokio::sync::RwLock;
use tower_lsp::{
    Client, LanguageServer,
    jsonrpc::Result,
    lsp_types::*,
};

pub struct Backend {
    client: Client,
    docs: RwLock<HashMap<Url, String>>,
}

impl Backend {
    pub fn new(client: Client) -> Self {
        Backend {
            client,
            docs: RwLock::new(HashMap::new()),
        }
    }

    async fn run_diagnostics(&self, uri: Url, text: &str) {
        let diagnostics = compute_diagnostics(text);
        self.client.publish_diagnostics(uri, diagnostics, None).await;
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _params: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: None,
                    ..Default::default()
                }),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _params: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "pseudo_lsp initialized")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.text_document.text.clone();
        self.docs.write().await.insert(uri.clone(), text.clone());
        self.run_diagnostics(uri, &text).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        if let Some(change) = params.content_changes.into_iter().last() {
            let uri = params.text_document.uri;
            let text = change.text.clone();
            self.docs.write().await.insert(uri.clone(), text.clone());
            self.run_diagnostics(uri, &text).await;
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri;
        self.docs.write().await.remove(&uri);
        self.client.publish_diagnostics(uri, vec![], None).await;
    }

    async fn completion(
        &self,
        _params: CompletionParams,
    ) -> Result<Option<CompletionResponse>> {
        let items: Vec<CompletionItem> = KEYWORDS
            .iter()
            .map(|kw| CompletionItem {
                label: kw.to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                detail: Some("pseudo keyword".to_string()),
                ..Default::default()
            })
            .collect();
        Ok(Some(CompletionResponse::Array(items)))
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = &params.text_document_position_params.text_document.uri;
        let pos = params.text_document_position_params.position;

        let docs = self.docs.read().await;
        let text = match docs.get(uri) {
            Some(t) => t,
            None => return Ok(None),
        };

        let word = word_at(text, pos).to_lowercase();
        let description = keyword_description(&word);
        if description.is_empty() {
            return Ok(None);
        }

        Ok(Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: description.to_string(),
            }),
            range: None,
        }))
    }
}

fn compute_diagnostics(source: &str) -> Vec<Diagnostic> {
    let tokens = match Lexer::new(source).tokenize() {
        Err(e) => return to_diagnostic(&e),
        Ok(t) => t,
    };
    match Parser::new(tokens).parse() {
        Err(e) => to_diagnostic(&e),
        Ok(_) => vec![],
    }
}

fn to_diagnostic(err: &PseudoError) -> Vec<Diagnostic> {
    let line = err.line_of().unwrap_or(1).saturating_sub(1) as u32;
    vec![Diagnostic {
        range: Range {
            start: Position { line, character: 0 },
            end: Position { line, character: u32::MAX },
        },
        severity: Some(DiagnosticSeverity::ERROR),
        message: err.message(),
        source: Some("pseudo_lsp".to_string()),
        ..Default::default()
    }]
}

fn word_at(text: &str, pos: Position) -> &str {
    let lines: Vec<&str> = text.split('\n').collect();
    let line_idx = pos.line as usize;
    if line_idx >= lines.len() {
        return "";
    }
    let line = lines[line_idx];
    let col = pos.character as usize;
    if col > line.len() {
        return "";
    }

    let start = line[..col]
        .rfind(|c: char| !c.is_alphanumeric() && c != '_')
        .map(|i| i + 1)
        .unwrap_or(0);
    let end = line[col..]
        .find(|c: char| !c.is_alphanumeric() && c != '_')
        .map(|i| i + col)
        .unwrap_or(line.len());

    &line[start..end]
}

fn keyword_description(word: &str) -> &str {
    match word {
        "declare" => "**declare** `<name> as <type>`\n\nDeclares a variable with the given type (`integer`, `string`, `double`, `boolean`).",
        "if" => "**if** `<condition> then`\n\nConditional statement. Must be closed with `endif`.",
        "else" => "**else**\n\nAlternative branch of an `if` statement.",
        "elseif" => "**elseif** `<condition> then`\n\nAdditional conditional branch.",
        "endif" => "**endif**\n\nCloses an `if` block.",
        "while" => "**while** `<condition> do`\n\nLoop that runs while condition is true. Close with `endwhile`.",
        "endwhile" => "**endwhile**\n\nCloses a `while` loop.",
        "for" => "**for** `<var> = <start> to <end> [step <n>] do`\n\nCounted loop. Close with `endfor`.",
        "endfor" => "**endfor**\n\nCloses a `for` loop.",
        "function" => "**function** `<name>(<params>)`\n\nDefines a function. Close with `endfunction`.",
        "endfunction" => "**endfunction**\n\nCloses a `function` definition.",
        "return" => "**return** `[<expr>]`\n\nReturns a value from a function.",
        "print" => "**print** `<expr>`\n\nOutputs a value to stdout.",
        "input" => "**input** `<var>`\n\nReads a value from stdin into a variable.",
        "as" => "**as** `<type>`\n\nUsed with `declare` to specify the variable type.",
        "integer" => "**integer**\n\nInteger data type (64-bit signed).",
        "double" => "**double**\n\nFloating-point data type (64-bit).",
        "string" => "**string**\n\nString data type.",
        "boolean" => "**boolean**\n\nBoolean data type (`true` or `false`).",
        "true" => "**true**\n\nBoolean literal.",
        "false" => "**false**\n\nBoolean literal.",
        "and" => "**and**\n\nLogical AND operator.",
        "or" => "**or**\n\nLogical OR operator.",
        "not" => "**not** `<expr>`\n\nLogical NOT operator.",
        "break" => "**break**\n\nExits the current loop.",
        "continue" => "**continue**\n\nSkips to the next loop iteration.",
        "step" => "**step** `<n>`\n\nSpecifies the increment for a `for` loop.",
        "to" => "**to** `<end>`\n\nSpecifies the end value in a `for` loop.",
        _ => "",
    }
}

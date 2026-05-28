use std::borrow::Cow;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::{Context, Result};
use brain_core::{git::link::git_head, BrainStore};
use rmcp::{
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{
        CallToolResult, Content, ErrorCode, ErrorData as McpError, ServerCapabilities, ServerInfo,
    },
    schemars::JsonSchema,
    tool, tool_handler, tool_router,
    transport::stdio,
    ServerHandler, ServiceExt,
};
use serde::Deserialize;

#[derive(Clone)]
pub struct BrainMcp {
    store: Arc<BrainStore>,
    #[allow(dead_code)] // used by #[tool_handler] via rmcp macros
    tool_router: ToolRouter<Self>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct CheckpointArgs {
    /// Short milestone or handoff note.
    note: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct DecisionArgs {
    /// Decision text to remember across tools.
    text: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct TaskArgs {
    /// Task title. Omit when clearing.
    #[serde(default)]
    title: Option<String>,
    /// Set true to clear the active task.
    #[serde(default)]
    clear: bool,
}

impl BrainMcp {
    pub fn new(repo_root: PathBuf) -> Result<Self> {
        let store = BrainStore::new(&repo_root);
        ensure_initialized(&store)?;
        Ok(Self {
            store: Arc::new(store),
            tool_router: Self::tool_router(),
        })
    }

    fn repo_root(&self) -> &Path {
        self.store.repo_root()
    }

    async fn run_store<F, T>(&self, f: F) -> Result<T, McpError>
    where
        F: FnOnce(BrainStore) -> Result<T> + Send + 'static,
        T: Send + 'static,
    {
        let store = BrainStore::new(self.repo_root());
        tokio::task::spawn_blocking(move || f(store))
            .await
            .map_err(|e| internal_err(format!("task join failed: {e}")))?
            .map_err(|e| internal_err(e.to_string()))
    }
}

#[tool_router]
impl BrainMcp {
    #[tool(description = "Print the shared handoff brief for the current repository scope.")]
    async fn brain_resume(&self) -> Result<CallToolResult, McpError> {
        let brief = self.run_store(|store| store.resume()).await?;
        let task = brief.active_task.unwrap_or_else(|| "<none>".to_string());
        let mut text =
            format!("# oerum-agent handoff\n\n**Active task:** {task}\n\n**Recent decisions:**\n");
        if brief.top_decisions.is_empty() {
            text.push_str("- (none)\n");
        } else {
            for d in brief.top_decisions {
                text.push_str(&format!("- {d}\n"));
            }
        }
        text.push_str("\n**Suggested next actions:**\n");
        for a in brief.next_actions {
            text.push_str(&format!("- {a}\n"));
        }
        Ok(CallToolResult::success(vec![Content::text(text)]))
    }

    #[tool(description = "Append a checkpoint note at a meaningful milestone.")]
    async fn brain_checkpoint(
        &self,
        Parameters(args): Parameters<CheckpointArgs>,
    ) -> Result<CallToolResult, McpError> {
        let note = args.note;
        let message = self
            .run_store(|store| {
                let result = store.checkpoint(note)?;
                Ok(result.message)
            })
            .await?;
        Ok(CallToolResult::success(vec![Content::text(message)]))
    }

    #[tool(description = "Record a key decision others should know.")]
    async fn brain_decision(
        &self,
        Parameters(args): Parameters<DecisionArgs>,
    ) -> Result<CallToolResult, McpError> {
        let text = args.text;
        let message = self
            .run_store(|store| {
                let result = store.record_decision(text)?;
                Ok(result.message)
            })
            .await?;
        Ok(CallToolResult::success(vec![Content::text(message)]))
    }

    #[tool(description = "Set or clear the active task for this repository.")]
    async fn brain_task(
        &self,
        Parameters(args): Parameters<TaskArgs>,
    ) -> Result<CallToolResult, McpError> {
        let value = if args.clear { None } else { args.title };
        if !args.clear && value.is_none() {
            return Err(McpError {
                code: ErrorCode::INVALID_PARAMS,
                message: Cow::from("provide `title` or set `clear` to true"),
                data: None,
            });
        }
        let message = self
            .run_store(|store| {
                let result = store.record_task(value)?;
                Ok(result.message)
            })
            .await?;
        Ok(CallToolResult::success(vec![Content::text(message)]))
    }

    #[tool(description = "Show resolved brain storage paths for the current repository.")]
    async fn brain_scope(&self) -> Result<CallToolResult, McpError> {
        let root = self.repo_root().display().to_string();
        let brain_dir = self.repo_root().join(".brain").display().to_string();
        let snapshot = self.run_store(|store| store.load_snapshot()).await.ok();
        let git = snapshot
            .and_then(|s| s.git_head)
            .or_else(|| git_head(self.repo_root()));
        let text = format!(
            "repo_root: {root}\nbrain_dir: {brain_dir}\ngit_head: {}",
            git.unwrap_or_else(|| "<unknown>".to_string())
        );
        Ok(CallToolResult::success(vec![Content::text(text)]))
    }
}

#[tool_handler]
impl ServerHandler for BrainMcp {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build()).with_instructions(
            "Shared session brain for cross-tool continuity. Call brain_resume at session start and brain_checkpoint after meaningful progress.",
        )
    }
}

pub async fn run_stdio() -> Result<()> {
    let repo_root = resolve_repo_root()?;
    let service = BrainMcp::new(repo_root)?
        .serve(stdio())
        .await
        .context("failed to start MCP stdio server")?;
    service.waiting().await.context("MCP server exited")?;
    Ok(())
}

fn resolve_repo_root() -> Result<PathBuf> {
    if let Ok(raw) = std::env::var("BRAIN_REPO_ROOT") {
        let path = PathBuf::from(raw);
        if path.is_dir() {
            return Ok(path);
        }
    }
    std::env::current_dir().context("could not resolve current directory")
}

fn ensure_initialized(store: &BrainStore) -> Result<()> {
    let snapshot_path = store.repo_root().join(".brain").join("snapshot.json");
    if snapshot_path.exists() {
        return Ok(());
    }
    let head = git_head(store.repo_root());
    store.init(head)?;
    Ok(())
}

fn internal_err(message: String) -> McpError {
    McpError {
        code: ErrorCode::INTERNAL_ERROR,
        message: Cow::from(message),
        data: None,
    }
}

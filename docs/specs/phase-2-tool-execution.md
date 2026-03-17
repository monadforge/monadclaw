# Phase 2 — Tool Execution

**Goal:** agent can call tools (bash, file read/write) inside a turn.
Requires Phase 1 (session persistence).

## New Crate: `crates/tools`

```
crates/tools/src/
├── lib.rs
├── tool.rs            # Tool trait, ToolCall, ToolResult
├── registry.rs        # ToolRegistry — register + dispatch
└── builtin/
    ├── mod.rs
    ├── bash.rs        # run shell command, return stdout/stderr + exit code
    ├── read_file.rs   # read a file from the filesystem
    └── write_file.rs  # write content to a file
```

## Core Types

```rust
/// A tool the agent can call.
#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    /// JSON Schema describing the input parameters (used in LLM tool spec).
    fn schema(&self) -> serde_json::Value;
    async fn call(&self, args: serde_json::Value) -> ToolResult;
}

pub struct ToolResult {
    pub output: String,
    pub is_error: bool,
}

/// Registry of available tools.
pub struct ToolRegistry {
    tools: HashMap<String, Arc<dyn Tool>>,
}

impl ToolRegistry {
    pub fn new() -> Self
    pub fn register(&mut self, tool: impl Tool + 'static)
    /// JSON Schema array for the LLM `tools` parameter.
    pub fn schemas(&self) -> Vec<serde_json::Value>
    pub async fn dispatch(&self, name: &str, args: serde_json::Value) -> ToolResult
}
```

## Built-in Tools

### `bash`
- Args: `{ "command": "ls -la" }`
- Returns: stdout + stderr, exit code in output
- Security: configurable allowlist; disabled by default for remote sessions

### `read_file`
- Args: `{ "path": "/path/to/file" }`
- Returns: file content as text (up to size limit)
- Security: path restricted to allowed directories (configurable)

### `write_file`
- Args: `{ "path": "/path/to/file", "content": "..." }`
- Returns: confirmation or error
- Security: same path restrictions as read_file

## Agent Loop Changes (`crates/agent`)

The loop in `run_turn` becomes iterative:

```
loop (max MAX_ITERATIONS = 10):
  1. Call provider with tool schemas attached
  2. If response is text only → commit + return
  3. If response contains tool_calls:
     a. For each call: dispatch to ToolRegistry
     b. Append assistant message (with tool_calls) to history
     c. Append each tool result as a tool message
     d. Continue loop
```

New function signature:

```rust
pub async fn run_turn(
    session: &mut AgentSession,
    input: impl Into<String>,
    provider: &dyn Provider,
    tools: Option<&ToolRegistry>,   // None = no tools
) -> Result<String, AgentError>
```

## Provider Trait Changes

```rust
pub trait Provider: Send + Sync {
    fn stream(&self, messages: Vec<ChatMessage>) -> Pin<Box<dyn Stream<Item = ProviderEvent> + Send>>;

    /// Stream with tool definitions attached. Default: ignore tools, fall back to stream().
    fn stream_with_tools(
        &self,
        messages: Vec<ChatMessage>,
        tools: Vec<serde_json::Value>,
    ) -> Pin<Box<dyn Stream<Item = ProviderEvent> + Send>> {
        self.stream(messages)
    }

    fn supports_tools(&self) -> bool { false }
}
```

New `ProviderEvent` variants:

```rust
pub enum ProviderEvent {
    Delta(String),
    ToolCallStart { id: String, name: String },
    ToolCallArgsDelta { id: String, delta: String },
    ToolCallDone { id: String },
    Done,
    Error(String),
}
```

## Config

```toml
[tools]
enabled = ["bash", "read_file", "write_file"]
allowed_paths = ["/home/user/projects"]   # for file tools
```

## Dashboard Changes

- Chat bubbles show tool calls inline (collapsible)
- Show tool name + args + result

## Deliverable

- Agent can execute shell commands, read and write files
- Full agentic loop: think → act → observe → respond

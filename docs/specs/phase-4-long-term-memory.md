# Phase 4 — Long-term Memory

**Goal:** agent remembers facts across sessions (user preferences, past events, recurring context).
Requires Phase 1 (sessions).

## New Crate: `crates/memory`

```
crates/memory/src/
├── lib.rs
├── store.rs       # SQLite FTS — write + search memory entries
├── extractor.rs   # extract notable facts from a completed turn
└── injector.rs    # inject relevant memories into system prompt
```

## Storage

SQLite FTS5 table, co-located in the same database as sessions:

```sql
CREATE VIRTUAL TABLE memories USING fts5(
    content,                      -- the memory text
    source        UNINDEXED,      -- 'user' | 'agent' | 'tool'
    session_id    UNINDEXED,      -- origin session
    created_at    UNINDEXED
);
```

## Public API (`MemoryStore`)

```rust
impl MemoryStore {
    /// Save a memory entry.
    pub async fn save(&self, content: &str, source: &str, session_id: &str) -> Result<()>

    /// Full-text search, returns top N results ordered by relevance.
    pub async fn search(&self, query: &str, limit: usize) -> Result<Vec<MemoryEntry>>

    /// Delete all memories from a session.
    pub async fn delete_by_session(&self, session_id: &str) -> Result<()>

    /// List all memories (for dashboard viewer).
    pub async fn list(&self, limit: usize, offset: usize) -> Result<Vec<MemoryEntry>>
}
```

## Memory Extraction

After each completed turn, run a lightweight extraction pass:

```rust
pub async fn extract_memories(
    user_msg: &str,
    assistant_reply: &str,
    provider: &dyn Provider,
) -> Vec<String>
```

Extraction prompt (short, cheap model preferred):

```
From this conversation turn, extract 0-3 notable facts worth remembering
long-term (preferences, names, decisions, recurring topics).
Output one fact per line. If nothing notable, output nothing.

User: {user_msg}
Assistant: {assistant_reply}
```

Facts are saved as individual memory entries.

## Memory Injection

At the start of each turn, search for relevant memories and inject into
the system prompt:

```rust
pub fn build_system_prompt(base: &str, memories: &[MemoryEntry]) -> String {
    if memories.is_empty() {
        return base.to_string();
    }
    format!(
        "{base}\n\n## What you remember about the user\n{}",
        memories.iter().map(|m| format!("- {}", m.content)).collect::<Vec<_>>().join("\n")
    )
}
```

## Config

```toml
[memory]
enabled         = true
search_limit    = 5       # how many memories to inject per turn
extract_enabled = true    # whether to extract memories after each turn
```

## API Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/v1/memory` | GET | List all memories (paginated) |
| `/api/v1/memory/search` | GET | `?q=query` — search memories |
| `/api/v1/memory/:id` | DELETE | Delete a specific memory |

## Dashboard Changes

- New "Memory" page: searchable list of all stored facts
- Delete individual entries
- Indicator on chat messages that triggered memory saves

## Deliverable

- Agent accumulates long-term context across sessions
- Relevant memories automatically injected at conversation start
- Dashboard lets you inspect and delete stored memories

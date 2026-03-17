# Phase 1 — Session Persistence + System Prompt

**Goal:** conversations survive server restarts; agent has a configurable identity.

## Config

```toml
# config.toml additions
system_prompt = "You are a helpful personal assistant."
```

## New Crate: `crates/store`

```
crates/store/src/
├── lib.rs
├── db.rs          # sqlx pool setup, migrations
└── session.rs     # Session CRUD + message history
```

### Schema

```sql
CREATE TABLE sessions (
    id          TEXT PRIMARY KEY,
    created_at  INTEGER NOT NULL,
    updated_at  INTEGER NOT NULL
);

CREATE TABLE messages (
    id          TEXT PRIMARY KEY,
    session_id  TEXT NOT NULL REFERENCES sessions(id),
    role        TEXT NOT NULL,   -- 'user' | 'assistant' | 'system'
    content     TEXT NOT NULL,
    created_at  INTEGER NOT NULL
);
```

### Public API (`SessionStore`)

```rust
impl SessionStore {
    pub async fn create_session(&self) -> Result<Session>
    pub async fn get_session(&self, id: &str) -> Result<Option<Session>>
    pub async fn list_sessions(&self) -> Result<Vec<Session>>
    pub async fn append_message(&self, session_id: &str, role: Role, content: &str) -> Result<()>
    pub async fn get_messages(&self, session_id: &str) -> Result<Vec<StoredMessage>>
    pub async fn delete_session(&self, id: &str) -> Result<()>
}
```

## API Changes

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/v1/sessions` | POST | Create session → `{ id, created_at }` |
| `/api/v1/sessions` | GET | List sessions → `[{ id, created_at, updated_at }]` |
| `/api/v1/sessions/:id` | GET | Session metadata |
| `/api/v1/sessions/:id/messages` | GET | Full message history |
| `/api/v1/sessions/:id/chat` | POST | Send message, stream reply |
| `/api/v1/chat` | POST | Keep as stateless fallback (no session ID) |

## AppState Changes

```rust
pub struct AppState {
    pub config:             Arc<Config>,
    pub provider:           Arc<dyn Provider>,
    pub store:              Arc<SessionStore>,    // new
    pub dashboard_password: Option<Arc<String>>,
}
```

## Chat Route Logic (`POST /api/v1/sessions/:id/chat`)

1. Load message history from store
2. Reconstruct `AgentSession` (replay history + prepend system prompt from config)
3. Call `session.begin_turn(input)`
4. Stream `provider.stream(turn.messages)` as SSE
5. On stream complete: persist user + assistant messages to store

## Dashboard Changes

- Sidebar lists sessions (fetch `GET /api/v1/sessions`)
- Chat sends to `/api/v1/sessions/:id/chat`
- "New chat" button → `POST /api/v1/sessions`
- Active session highlighted in sidebar

## Dependencies

- `sqlx` with `sqlite` + `runtime-tokio` features
- `ulid` or `uuid` for session IDs

## Deliverable

- Conversations persist across server restarts
- Agent has a fixed identity via `system_prompt` in config
- Dashboard shows conversation history per session

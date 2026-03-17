# Phase 3 — Discord Bot

**Goal:** talk to the agent through Discord DMs and channel mentions.
Requires Phase 1 (sessions). Phase 2 (tools) optional but recommended.

## New Crate: `crates/discord`

```
crates/discord/src/
├── lib.rs
├── bot.rs         # serenity Client setup, event dispatch
├── handler.rs     # EventHandler impl — on_message
└── reply.rs       # stream reply back to Discord (split long messages)
```

## Dependency

```toml
# serenity = async Discord API, feature-minimal
serenity = { version = "0.12", features = ["client", "gateway", "model", "rustls_backend"] }
```

## Session Mapping

Each Discord context maps to a persistent session:

| Context | Session key |
|---------|------------|
| DM | `discord:dm:{user_id}` |
| Channel mention | `discord:channel:{channel_id}` |
| Thread | `discord:thread:{thread_id}` |

On message received:
1. Look up session by key in `SessionStore`
2. Create if not found
3. Call `run_turn(session, message.content, provider, tools)`
4. Send reply (split at 2000 chars if needed)
5. Commit exchange to store

## Config

```toml
[discord]
bot_token_env    = "DISCORD_BOT_TOKEN"
# Allowlist — empty means bot ignores all DMs (safe default)
allowed_user_ids = ["123456789012345678"]
# Whether @mentions in servers are handled
handle_mentions  = true
```

## Security

- Allowlist checked before any agent call
- If `allowed_user_ids` is empty, bot does not respond (closed by default)
- Bot token loaded from env var only, never from config file

## Reply Strategy

Discord message limit = 2000 chars.

- If response ≤ 2000 chars: single message
- If response > 2000 chars: split at sentence/paragraph boundary, send sequentially
- Streaming not possible in Discord; collect full response then send

## App Entry Point

Two options (choose based on preference):

**Option A** — integrated: `apps/server/main.rs` spawns discord bot as a background tokio task alongside the HTTP server.

**Option B** — separate binary: `apps/bot/main.rs` is a standalone process that shares the SQLite store with the server.

Option A is simpler for a single-machine deployment.

## Deliverable

- Agent responds to DMs and @mentions on Discord
- Each Discord user/channel has its own persistent session
- Conversation history shared with dashboard (same SQLite store)

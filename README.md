# monadclaw

A minimalist personal AI agent framework, modular and extensible, written in Rust.
Personal project — not intended for production use.

**Translations:** [中文](docs/i18n/README.zh.md) · [Français](docs/i18n/README.fr.md)

---

## Quick Start

### Prerequisites

- Rust (stable) — [rustup.rs](https://rustup.rs)
- Node.js 18+ — for the dashboard
- An API key for a supported LLM provider (e.g. [OpenRouter](https://openrouter.ai/keys))

### 1. Configuration

Create `config.toml` in the project root (or at `~/.config/monadclaw/config.toml`):

```toml
active_provider = "openrouter"

[providers.openrouter]
model = "openai/gpt-4o-mini"
api_key_env = "OPENROUTER_API_KEY"
base_url = "https://openrouter.ai/api/v1/"
```

Create a `.env` file in the project root:

```env
OPENROUTER_API_KEY=sk-or-v1-...
```

### 2. Start the backend

```bash
# Load env and start the server
source .env && MONADCLAW_CONFIG=./config.toml cargo run
# Server starts on http://0.0.0.0:3000
```

### 3. Start the dashboard

```bash
cd dashboard
npm install
npm run dev
# Dashboard at http://localhost:5173
```

---

## Authentication

Monadclaw uses a **three-tier access model** based on connection origin and whether a password is configured.

| Connection | Password set? | Result |
|-----------|--------------|--------|
| Local (loopback) | No | ✅ Allow — no credentials needed |
| Local (loopback) | Yes | 🔑 Require Bearer token |
| Remote | No | ❌ 403 Forbidden |
| Remote | Yes | 🔑 Require Bearer token |

### Setting a password

Add `dashboard_password` to `config.toml` and restart the server:

```toml
dashboard_password = "your-secret-password"
```

The dashboard will show a login page and prompt for the password.
The token is stored in `localStorage` and has no expiry. To log out, clear browser storage.

### Remote access

Remote access is **blocked by default** when no password is set — a deliberate safety measure.
To enable it, set `dashboard_password` in the config.

> See [docs/auth.md](docs/auth.md) for the full auth policy.

---

## Project Structure

```
monadclaw/
├── apps/server/        # Binary entry point (Axum HTTP server)
├── crates/
│   ├── api/            # Axum router, routes, middleware
│   ├── chat/           # Chat message types
│   ├── config/         # TOML config loading
│   └── providers/      # LLM provider abstraction (genai)
├── dashboard/          # React 19 + TypeScript dashboard
├── docs/               # Internal specs and documentation
└── config.toml         # Local config (gitignored)
```

---

## Roadmap

| Feature | Status |
|---------|--------|
| TOML config loading with env key resolution | ✅ Done |
| LLM provider abstraction (genai) | ✅ Done |
| OpenAI-compatible custom endpoints (OpenRouter, Kimi, etc.) | ✅ Done |
| Streaming chat API (`POST /api/v1/chat`) | ✅ Done |
| Status API (`GET /api/v1/status`) | ✅ Done |
| Axum HTTP server with CORS | ✅ Done |
| React dashboard — shell, sidebar, navigation | ✅ Done |
| Chat page with streaming responses | ✅ Done |
| Three-tier auth middleware | ✅ Done |
| Dashboard login page + auth guard | ✅ Done |
| Agent loop (tool calls, multi-step reasoning) | 🔄 Planned |
| Short-term memory (conversation window) | 🔄 Planned |
| Long-term memory (persistent store) | 🔄 Planned |
| Discord bot interface | 🔄 Planned |
| Multiple LLM providers (Anthropic, Gemini, etc.) | 🔄 Planned |
| Config editor in dashboard | 🔄 Planned |
| Session history | 🔄 Planned |
| Usage tracking | 🔄 Planned |
| Logs viewer | 🔄 Planned |
| Cron / scheduled tasks | 🔄 Planned |
| Skills / extension system | 🔄 Planned |

---

## License

MIT

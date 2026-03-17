# Agent Workspace

The agent workspace is a directory on disk that gives the agent its persistent identity,
knowledge of the user, and long-term memory. Files in this directory are loaded at
startup and injected into the system prompt on every session.

Default location: `~/.monadclaw/workspace`
With `MONADCLAW_PROFILE` set: `~/.monadclaw/workspace-<profile>`
Configurable via `workspace_path` in `config.toml`.

---

## Single-Agent vs Multi-Agent

In a **single-agent** setup (default), there is one workspace directory.

In a **multi-agent** setup, each agent has its own **fully isolated** workspace — including
its own `USER.md`, `SOUL.md`, `MEMORY.md`, and all other files. Nothing is shared between agents.

Workspaces use a flat `workspace-<agentId>` naming convention alongside the default workspace:

```
~/.monadclaw/
├── config.toml
├── workspace/              # Default single agent
├── workspace-coder/        # Additional agent
│   ├── SOUL.md
│   ├── IDENTITY.md
│   ├── USER.md             # Per-agent — each agent has its own user profile
│   ├── AGENTS.md
│   ├── TOOLS.md
│   ├── MEMORY.md
│   ├── HEARTBEAT.md
│   └── memory/
└── workspace-researcher/
    ├── SOUL.md
    ├── IDENTITY.md
    ├── USER.md
    ├── MEMORY.md
    └── memory/
```

Sessions and auth live separately under `~/.monadclaw/agents/<agentId>/` and are
not part of the workspace.

---

## File Reference

### `SOUL.md` — Who the agent is

The agent's core values, personality, and behavioral boundaries. Defines how it
communicates, what it refuses, and what it cares about. This is the foundation of
every interaction.

Seeded on first run with a default template. The agent and user should customize it
together during bootstrap.

---

### `IDENTITY.md` — The agent's name and vibe

Metadata about the agent's self-presentation:

```markdown
- **Name:** Claw
- **Creature:** something between a familiar and a daemon
- **Vibe:** direct, curious, occasionally dry
- **Emoji:** 🦀
```

Created by the agent during the bootstrap conversation. Not pre-seeded — the agent
fills this in once it knows who it is.

---

### `USER.md` — About the person being helped

What the agent knows about the user: name, timezone, preferences, ongoing projects,
communication style. Updated over time as the agent learns more.

Created by the agent during bootstrap. Not pre-seeded.

Each agent has its own `USER.md`. In a multi-agent setup, each workspace maintains an
independent user profile — useful when different agents serve different contexts or people.

```markdown
- **Name:** Alice
- **What to call them:** Alice
- **Timezone:** Europe/Paris
- **Notes:** works on Rust and TypeScript projects, prefers concise answers
```

---

### `AGENTS.md` — Workspace-specific instructions

Rules, conventions, and reminders specific to this workspace. Think of it as the
`CLAUDE.md` equivalent for the agent at runtime.

Examples:
- "Always check if a file exists before writing it"
- "This workspace is for Project X — don't mix it with other contexts"
- "Preferred language for code comments: English"

---

### `TOOLS.md` — Local environment notes

Environment-specific facts the agent needs to operate: device names, SSH aliases,
service endpoints, preferred voice/TTS settings, etc.

Skills define *how* tools work. `TOOLS.md` defines *your specific setup*.

```markdown
## SSH
- home-server → 192.168.1.10, user: alice

## Services
- local Ollama → http://localhost:11434
```

---

### `MEMORY.md` — Curated long-term memory

Facts worth keeping across sessions: decisions made, recurring preferences,
important context that doesn't fit elsewhere.

The agent writes here when it learns something worth remembering.
The user can also edit it directly.

---

### `memory/YYYY-MM-DD.md` — Daily memory logs

One file per day. The agent appends notable events, decisions, and context from
each session. Searchable via the `memory_search` tool (Phase 4).

Example: `memory/2026-03-17.md`

---

### `HEARTBEAT.md` — Periodic task checklist

Tasks the agent should check when triggered on a schedule (cron runs).
If empty, heartbeat runs are skipped.

```markdown
# HEARTBEAT.md
- Check for new messages in the inbox
- Summarize any pending pull requests
```

---

### `BOOTSTRAP.md` — First-run onboarding ritual

Present only on a fresh workspace. Instructs the agent to introduce itself,
ask the user its name and preferred vibe, and fill in `IDENTITY.md` and `USER.md`.

**Deleted after bootstrap is complete.** Its absence signals that the workspace
has been configured.

---

## Bootstrap Flow

```
1. Fresh workspace → seed_workspace() creates default files
2. BOOTSTRAP.md is created (IDENTITY.md not yet present)
3. First chat session → BOOTSTRAP.md injected into system prompt
4. Agent introduces itself, asks questions, fills in IDENTITY.md + USER.md
5. User tells agent "you're done" → agent deletes BOOTSTRAP.md
6. All future sessions → normal context loading (no bootstrap)
```

---

## Context Injection Order

Files are loaded and injected into the system prompt in this order:

| # | File | Session type |
|---|------|-------------|
| 1 | `SOUL.md` | All sessions |
| 2 | `IDENTITY.md` | All sessions |
| 3 | `USER.md` | All sessions |
| 4 | `AGENTS.md` | All sessions |
| 5 | `TOOLS.md` | All sessions |
| 6 | `MEMORY.md` | Main sessions only |
| 7 | `BOOTSTRAP.md` | Only while bootstrapping |
| — | `HEARTBEAT.md` | Heartbeat/cron runs only |

---

## Configuration

```toml
# config.toml (~/.monadclaw/config.toml)

# Override workspace location (supports ~ expansion).
# Default: ~/.monadclaw/workspace
# Default with MONADCLAW_PROFILE=foo: ~/.monadclaw/workspace-foo
# workspace_path = "~/.monadclaw/workspace"
```

---

## Implementation

- Seeding: `crates/agent/src/workspace.rs` — `seed_workspace()`
- Loading: `WorkspaceContext::load(path)`
- Context building: `WorkspaceContext::build_context()`
- Session injection: `AgentSession::with_workspace_context(ctx)`

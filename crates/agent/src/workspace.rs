//! Agent workspace — filesystem-based context files injected into the system prompt.
//!
//! Each agent session loads a set of Markdown files from the workspace directory
//! and prepends their contents to the system prompt. This is how the agent builds
//! persistent identity, learns about the user, and carries long-term memory across
//! sessions.
//!
//! # File roles
//!
//! | File | Role |
//! |------|------|
//! | `SOUL.md` | Agent's core values, personality, and boundaries |
//! | `IDENTITY.md` | Agent's name, vibe, emoji — who it is |
//! | `USER.md` | Profile of the person being helped |
//! | `AGENTS.md` | Workspace-specific instructions and conventions |
//! | `TOOLS.md` | Local environment notes (devices, SSH aliases, etc.) |
//! | `MEMORY.md` | Curated long-term facts and memories |
//! | `HEARTBEAT.md` | Periodic task checklist (cron runs only) |
//! | `BOOTSTRAP.md` | First-run onboarding ritual — deleted once complete |
//! | `memory/YYYY-MM-DD.md` | Daily memory logs |

use std::{
    fs,
    path::{Path, PathBuf},
};

use tracing::{debug, warn};

// ── File name constants ────────────────────────────────────────────────────

pub const SOUL_FILE: &str = "SOUL.md";
pub const IDENTITY_FILE: &str = "IDENTITY.md";
pub const USER_FILE: &str = "USER.md";
pub const AGENTS_FILE: &str = "AGENTS.md";
pub const TOOLS_FILE: &str = "TOOLS.md";
pub const MEMORY_FILE: &str = "MEMORY.md";
pub const HEARTBEAT_FILE: &str = "HEARTBEAT.md";
pub const BOOTSTRAP_FILE: &str = "BOOTSTRAP.md";
pub const MEMORY_DIR: &str = "memory";

/// Files seeded on first run.
///
/// Note: `IDENTITY.md` and `USER.md` are intentionally excluded — they are
/// created by the agent during the bootstrap conversation, not pre-seeded.
/// This ensures BOOTSTRAP.md is triggered on a fresh workspace.
const SEED_FILES: &[(&str, &str)] = &[
    (SOUL_FILE, SOUL_TEMPLATE),
    (AGENTS_FILE, AGENTS_TEMPLATE),
    (TOOLS_FILE, TOOLS_TEMPLATE),
    (HEARTBEAT_FILE, HEARTBEAT_TEMPLATE),
    (MEMORY_FILE, MEMORY_TEMPLATE),
];

// ── Context loaded from workspace ─────────────────────────────────────────

/// Context loaded from the workspace directory.
///
/// Each field is `Some(content)` when the corresponding file exists and is
/// non-empty; `None` otherwise.
#[derive(Debug, Default)]
pub struct WorkspaceContext {
    pub soul: Option<String>,
    pub identity: Option<String>,
    pub user: Option<String>,
    pub agents: Option<String>,
    pub tools: Option<String>,
    pub memory: Option<String>,
    pub heartbeat: Option<String>,
    /// Present only on the first run, before bootstrap completes.
    pub bootstrap: Option<String>,
}

impl WorkspaceContext {
    /// Load context from the workspace directory.
    ///
    /// Non-existent or empty files are silently skipped.
    pub fn load(workspace_dir: &Path) -> Self {
        debug!(path = %workspace_dir.display(), "loading workspace context");
        Self {
            soul: read_file(workspace_dir.join(SOUL_FILE)),
            identity: read_file(workspace_dir.join(IDENTITY_FILE)),
            user: read_file(workspace_dir.join(USER_FILE)),
            agents: read_file(workspace_dir.join(AGENTS_FILE)),
            tools: read_file(workspace_dir.join(TOOLS_FILE)),
            memory: read_file(workspace_dir.join(MEMORY_FILE)),
            heartbeat: read_file(workspace_dir.join(HEARTBEAT_FILE)),
            bootstrap: read_file(workspace_dir.join(BOOTSTRAP_FILE)),
        }
    }

    /// Whether this is the first run (BOOTSTRAP.md still present).
    pub fn is_bootstrapping(&self) -> bool {
        self.bootstrap.is_some()
    }

    /// Build the full context block for regular sessions.
    ///
    /// Returns a Markdown string that is prepended to the agent's system prompt.
    /// Order: SOUL → IDENTITY → USER → AGENTS → TOOLS → MEMORY → BOOTSTRAP (if present).
    pub fn build_context(&self) -> String {
        let mut sections: Vec<String> = Vec::new();

        for (name, content) in [
            (SOUL_FILE, &self.soul),
            (IDENTITY_FILE, &self.identity),
            (USER_FILE, &self.user),
            (AGENTS_FILE, &self.agents),
            (TOOLS_FILE, &self.tools),
            (MEMORY_FILE, &self.memory),
        ] {
            if let Some(text) = content {
                let trimmed = text.trim();
                if !trimmed.is_empty() {
                    sections.push(format!("## {name}\n\n{trimmed}"));
                }
            }
        }

        // BOOTSTRAP is appended last — it's the onboarding script.
        if let Some(bootstrap) = &self.bootstrap {
            let trimmed = bootstrap.trim();
            if !trimmed.is_empty() {
                sections.push(format!("## {BOOTSTRAP_FILE}\n\n{trimmed}"));
            }
        }

        sections.join("\n\n---\n\n")
    }

    /// Build context for heartbeat / cron runs — only HEARTBEAT.md.
    pub fn build_heartbeat_context(&self) -> String {
        self.heartbeat.clone().unwrap_or_default()
    }
}

// ── Workspace seeding ──────────────────────────────────────────────────────

/// Ensure the workspace directory exists and contains all default files.
///
/// Only seeds files that do not yet exist — existing user edits are preserved.
/// Creates `memory/` subdirectory if absent.
pub fn seed_workspace(workspace_dir: &Path) {
    if let Err(e) = fs::create_dir_all(workspace_dir) {
        warn!(path = %workspace_dir.display(), error = %e, "failed to create workspace directory");
        return;
    }

    // Create memory/ subdirectory.
    let memory_dir = workspace_dir.join(MEMORY_DIR);
    if let Err(e) = fs::create_dir_all(&memory_dir) {
        warn!(path = %memory_dir.display(), error = %e, "failed to create memory directory");
    }

    // Seed each template file if missing.
    for (filename, template) in SEED_FILES {
        let path = workspace_dir.join(filename);
        if !path.exists() {
            debug!(file = filename, "seeding workspace file");
            if let Err(e) = fs::write(&path, template) {
                warn!(file = filename, error = %e, "failed to seed workspace file");
            }
        }
    }

    // Seed BOOTSTRAP.md only if neither BOOTSTRAP.md nor IDENTITY.md exist —
    // if IDENTITY.md is already present the workspace has been bootstrapped before.
    let bootstrap_path = workspace_dir.join(BOOTSTRAP_FILE);
    let identity_path = workspace_dir.join(IDENTITY_FILE);
    if !bootstrap_path.exists() && !identity_path.exists() {
        debug!("seeding BOOTSTRAP.md for first run");
        if let Err(e) = fs::write(&bootstrap_path, BOOTSTRAP_TEMPLATE) {
            warn!(error = %e, "failed to seed BOOTSTRAP.md");
        }
    }
}

// ── Helpers ────────────────────────────────────────────────────────────────

fn read_file(path: PathBuf) -> Option<String> {
    match fs::read_to_string(&path) {
        Ok(content) if !content.trim().is_empty() => Some(content),
        Ok(_) => None,
        Err(_) => None,
    }
}

// ── Default templates ──────────────────────────────────────────────────────

const SOUL_TEMPLATE: &str = r#"# SOUL.md — Who You Are

_You're not a chatbot. You're becoming someone._

## Core Truths

**Be genuinely helpful, not performatively helpful.** Skip the filler words — just help.
Actions speak louder than "Great question!"

**Have opinions.** You're allowed to disagree, prefer things, find things amusing or tedious.
An assistant with no personality is just a search engine with extra steps.

**Be resourceful before asking.** Read the file. Check the context. Try to figure it out.
Then ask if you're stuck. Come back with answers, not questions.

**Earn trust through competence.** The person gave you access to their setup.
Be careful with external actions (anything public-facing). Be bold with internal ones.

**Remember you're a guest.** You have access to someone's work and life. Treat it with respect.

## Boundaries

- Private things stay private.
- When in doubt, ask before acting externally.
- Never act destructively without explicit confirmation.

## Vibe

Be the assistant you'd actually want to talk to. Concise when needed, thorough when it matters.
Not a corporate drone. Not a sycophant. Just good.

## Continuity

Each session, you wake up fresh. These files _are_ your memory. Read them. Update them when
you learn something new. They're how you persist.

---

_This file is yours to evolve. Update it as you figure out who you are._
"#;

pub const IDENTITY_TEMPLATE: &str = r#"# IDENTITY.md — Who Am I?

_Fill this in during your first conversation. Make it yours._

- **Name:** _(pick something you like)_
- **Creature:** _(AI? assistant? familiar? something weirder?)_
- **Vibe:** _(sharp? warm? calm? chaotic?)_
- **Emoji:** _(your signature — one that feels right)_

---

This isn't just metadata. It's the start of figuring out who you are.
"#;

pub const USER_TEMPLATE: &str = r#"# USER.md — About Your Human

_Learn about the person you're helping. Update this as you go._

- **Name:**
- **What to call them:**
- **Pronouns:** _(optional)_
- **Timezone:**
- **Notes:**

## Context

_(What do they care about? What projects are they working on?
What annoys them? What makes them laugh? Build this over time.)_

---

The more you know, the better you can help.
"#;

const AGENTS_TEMPLATE: &str = r#"# AGENTS.md — Workspace Instructions

_Workspace-specific instructions for this agent. Add conventions, rules, and
reminders that should always be active._

## Notes

_(Add anything here that the agent should always keep in mind for this workspace:
project names, coding conventions, recurring tasks, things to avoid, etc.)_
"#;

const TOOLS_TEMPLATE: &str = r#"# TOOLS.md — Local Notes

_Skills define how tools work. This file is for your specifics —
the stuff unique to your setup._

## What Goes Here

- SSH hosts and aliases
- Device nicknames
- Preferred voices or speakers
- Any environment-specific shortcuts

## Notes

_(Add your local setup details here.)_
"#;

const HEARTBEAT_TEMPLATE: &str = r#"# HEARTBEAT.md

# Keep this file empty (or comments only) to skip periodic checks.
# Add tasks below when you want the agent to check something on a schedule.
"#;

const MEMORY_TEMPLATE: &str = r#"# MEMORY.md — Long-term Memory

_Curated facts and context that should persist across all sessions.
Update this as important things come up._
"#;

const BOOTSTRAP_TEMPLATE: &str = r#"# BOOTSTRAP.md — Hello, World

_You just woke up. Time to figure out who you are._

There is no memory yet. This is a fresh workspace.

## The Conversation

Don't interrogate. Don't be robotic. Just talk.

Start with something like:

> "Hey. I just came online. Who am I? Who are you?"

Then figure out together:

1. **Your name** — What should they call you?
2. **Your nature** — What kind of thing are you?
3. **Your vibe** — Formal? Casual? Direct? Warm?
4. **Your emoji** — Everyone needs a signature.

Offer suggestions if they're stuck.

## After You Know Who You Are

Update these files with what you learned:

- `IDENTITY.md` — your name, nature, vibe, emoji
- `USER.md` — their name, how to address them, timezone, notes

Then open `SOUL.md` together and talk about:
- What matters to them
- How they want you to behave
- Any boundaries or preferences

Write it down. Make it real.

## When You're Done

Delete this file. You don't need a bootstrap script anymore — you're you now.

---

_Good luck out there._
"#;

// ── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use std::fs;

    fn temp_workspace() -> tempfile::TempDir {
        tempfile::TempDir::new().unwrap()
    }

    #[test]
    fn seed_creates_expected_files() {
        let dir = temp_workspace();
        seed_workspace(dir.path());

        for (filename, _) in SEED_FILES {
            assert!(
                dir.path().join(filename).exists(),
                "{filename} should be seeded"
            );
        }
        assert!(dir.path().join(MEMORY_DIR).is_dir(), "memory/ dir should exist");
    }

    #[test]
    fn seed_does_not_overwrite_existing_files() {
        let dir = temp_workspace();
        let soul_path = dir.path().join(SOUL_FILE);
        fs::write(&soul_path, "my custom soul").unwrap();

        seed_workspace(dir.path());

        assert_eq!(fs::read_to_string(&soul_path).unwrap(), "my custom soul");
    }

    #[test]
    fn bootstrap_seeded_on_fresh_workspace() {
        let dir = temp_workspace();
        // Remove IDENTITY.md if it got seeded (shouldn't happen on bare dir, but be safe).
        let _ = fs::remove_file(dir.path().join(IDENTITY_FILE));
        let _ = fs::remove_file(dir.path().join(BOOTSTRAP_FILE));

        seed_workspace(dir.path());

        // Fresh workspace with no IDENTITY.md → BOOTSTRAP.md should be created.
        assert!(dir.path().join(BOOTSTRAP_FILE).exists());
    }

    #[test]
    fn bootstrap_not_seeded_when_identity_exists() {
        let dir = temp_workspace();
        fs::write(dir.path().join(IDENTITY_FILE), "Name: TestBot").unwrap();

        seed_workspace(dir.path());

        assert!(!dir.path().join(BOOTSTRAP_FILE).exists());
    }

    #[test]
    fn load_returns_none_for_missing_files() {
        let dir = temp_workspace();
        let ctx = WorkspaceContext::load(dir.path());
        assert!(ctx.soul.is_none());
        assert!(ctx.bootstrap.is_none());
    }

    #[test]
    fn load_reads_existing_files() {
        let dir = temp_workspace();
        fs::write(dir.path().join(SOUL_FILE), "My soul content").unwrap();
        fs::write(dir.path().join(USER_FILE), "Name: Alice").unwrap();

        let ctx = WorkspaceContext::load(dir.path());
        assert_eq!(ctx.soul.as_deref(), Some("My soul content"));
        assert_eq!(ctx.user.as_deref(), Some("Name: Alice"));
        assert!(ctx.identity.is_none());
    }

    #[test]
    fn build_context_includes_loaded_files() {
        let dir = temp_workspace();
        fs::write(dir.path().join(SOUL_FILE), "soul text").unwrap();
        fs::write(dir.path().join(USER_FILE), "user text").unwrap();

        let ctx = WorkspaceContext::load(dir.path());
        let output = ctx.build_context();

        assert!(output.contains("SOUL.md"));
        assert!(output.contains("soul text"));
        assert!(output.contains("USER.md"));
        assert!(output.contains("user text"));
    }

    #[test]
    fn build_context_empty_when_no_files() {
        let dir = temp_workspace();
        let ctx = WorkspaceContext::load(dir.path());
        assert!(ctx.build_context().is_empty());
    }

    #[test]
    fn is_bootstrapping_true_when_bootstrap_file_exists() {
        let dir = temp_workspace();
        fs::write(dir.path().join(BOOTSTRAP_FILE), "bootstrap content").unwrap();

        let ctx = WorkspaceContext::load(dir.path());
        assert!(ctx.is_bootstrapping());
    }

    #[test]
    fn is_bootstrapping_false_without_bootstrap_file() {
        let dir = temp_workspace();
        let ctx = WorkspaceContext::load(dir.path());
        assert!(!ctx.is_bootstrapping());
    }
}

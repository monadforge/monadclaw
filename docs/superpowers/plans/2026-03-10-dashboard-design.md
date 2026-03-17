# Dashboard Design — Monadclaw

**Date:** 2026-03-10
**Status:** Approved
**References:** `tmp/openclaw/ui` (features/layout), `tmp/website` (visual style)
**Tech:** Vite + React 19 + TypeScript

---

## 1. Visual Design System

### Color Palette (from `tmp/website`)

```css
--color-bg:          #0a0a0a;   /* pure black — page background */
--color-surface:     #111111;   /* card / panel background */
--color-surface-2:   #1a1a1a;   /* elevated surface (modals, dropdowns) */
--color-border:      #2a2a2a;   /* subtle borders */
--color-accent:      #00ff88;   /* neon green — primary accent */
--color-accent-dim:  #00cc6a;   /* hover / pressed state */
--color-text:        #e2e8f0;   /* primary text */
--color-text-muted:  #4a5568;   /* secondary / placeholder text */

/* Semantic */
--color-success:     #00ff88;   /* same as accent */
--color-warning:     #f59e0b;
--color-error:       #ef4444;
--color-info:        #22d3ee;
```

### Typography (from `tmp/website`)

- **Headings / data / monospace:** JetBrains Mono
- **Body / labels / UI text:** Inter
- **Base size:** 14px, line-height 1.55
- **Nav / status labels:** uppercase, 11px, letter-spacing 0.08em

### Spacing & Shape

- **Border radius:** 8px (default), 4px (small chips), 12px (modals)
- **Sidebar width:** 220px (collapsed: 56px icon-only)
- **Topbar height:** 56px
- **Content padding:** 24px
- **Card gap:** 16px

### Motion

- Fast: 120ms ease-out (hover states)
- Normal: 200ms ease-in-out (panel transitions)
- Terminal blink: 1s step-end infinite (Chat cursor)

---

## 2. Shell Layout (from openclaw)

```
┌────────────────────────────────────────────────────┐
│  Topbar (56px) — logo [monadclaw] · status pill · agent selector  │
├──────────┬─────────────────────────────────────────┤
│          │                                         │
│ Sidebar  │  <Page content>                         │
│ (220px)  │                                         │
│          │                                         │
│ nav      │                                         │
│ groups   │                                         │
│          │                                         │
└──────────┴─────────────────────────────────────────┘
```

- Topbar: `[monadclaw]` logo (bracket style from website), global status pill, active provider badge
- Sidebar: collapsible groups, icon + label, neon green active indicator
- Content: scrollable, max-width 1200px centered for wide screens

---

## 3. Pages

### Navigation Groups

```
─── MONITOR ───
  Overview
  Logs
  Sessions
  Usage

─── MANAGE ───
  Agents
  Channels
  Skills
  Cron

─── SETTINGS ───
  Config

─── INTERACT ───
  Chat
```

### Page Specs

#### Overview
Status dashboard. Cards showing: agent health, active provider, memory usage (short/long-term), message count today. Sparkline charts for recent activity. Alert banner if agent is offline.

#### Chat
Split view: left = conversation thread, right = tool call output panel (collapsible). Markdown rendering, terminal blink cursor on input. Message grouping by turn. Auto-scroll.

#### Channels
Table of configured interfaces (Discord, REST API, etc.) with status indicators. Add/remove channels. Per-channel config panel.

#### Agents
Agent detail view: tabs for Files, Tools, Config, Channels, Cron. One agent per monadclaw instance (no multi-agent list needed at v1).

#### Config
JSON editor with syntax highlighting + optional form view toggle. Save / discard buttons. Read-only sensitive fields.

#### Sessions
Filterable table of past conversations: timestamp, channel, message count, provider used. Click to expand session transcript.

#### Logs
Real-time log stream with level filter (DEBUG / INFO / WARN / ERROR) and text search. Auto-scroll toggle. Neon green for INFO, amber for WARN, red for ERROR.

#### Usage
Token usage by provider, by day. Cost estimate. Bar chart per model. Date range picker.

#### Cron
Table of scheduled jobs with next-run time, last-run status, enable/disable toggle. Log of recent executions.

#### Skills
List of installed plugins/skills with version, enabled state, config panel. Install from path.

---

## 4. Component Architecture

```
dashboard/src/
├── main.tsx
├── App.tsx                  # router, shell layout
├── components/
│   ├── layout/
│   │   ├── Topbar.tsx
│   │   ├── Sidebar.tsx
│   │   └── Shell.tsx
│   ├── ui/
│   │   ├── StatusPill.tsx   # online/offline/degraded
│   │   ├── Card.tsx
│   │   ├── Table.tsx
│   │   ├── Badge.tsx
│   │   ├── Button.tsx
│   │   └── JsonEditor.tsx
│   └── charts/
│       └── Sparkline.tsx
├── pages/
│   ├── OverviewPage.tsx
│   ├── ChatPage.tsx
│   ├── ChannelsPage.tsx
│   ├── AgentsPage.tsx
│   ├── ConfigPage.tsx
│   ├── SessionsPage.tsx
│   ├── LogsPage.tsx
│   ├── UsagePage.tsx
│   ├── CronPage.tsx
│   └── SkillsPage.tsx
├── api/
│   ├── client.ts            # fetch wrapper, auth header, error handling
│   ├── status.ts
│   ├── chat.ts
│   ├── channels.ts
│   ├── agents.ts
│   ├── config.ts
│   ├── sessions.ts
│   ├── logs.ts
│   ├── usage.ts
│   ├── cron.ts
│   └── skills.ts
├── hooks/
│   ├── useStatus.ts         # React Query polling
│   ├── useLogs.ts           # SSE or polling
│   └── useChat.ts
├── store/
│   └── uiStore.ts           # Zustand: sidebar collapse, theme
├── styles/
│   ├── tokens.css           # CSS custom properties
│   ├── base.css             # resets, body
│   └── layout.css           # shell grid
└── types/
    └── api.ts               # mirrors API response shapes
```

---

## 5. State Management

- **Server state:** React Query — polling for status/logs, mutation for config/cron
- **UI state:** Zustand — sidebar collapsed, active tab, log level filter
- **Chat:** local React state + React Query mutation for send

---

## 6. Key Dependencies

```json
{
  "react": "^19",
  "react-router-dom": "^7",
  "@tanstack/react-query": "^5",
  "zustand": "^5",
  "recharts": "^2",
  "monaco-editor": "^0.50"
}
```

Monaco for JSON config editor. Recharts for usage charts. No UI component library — custom components to match website aesthetic precisely.

---

## 7. API Integration

- Base URL: `/api/v1` (proxied to Rust server via Vite dev proxy)
- Auth: `Authorization: Bearer <token>` on all requests
- Errors: `{ error: { code, message } }` — displayed as toast/banner
- Logs: Server-Sent Events (`GET /api/v1/logs/stream`) or polling fallback

---

## 8. Out of Scope (v1)

- Multi-agent management
- i18n / localization
- Exec approval queue
- Multi-node / instance management
- Mobile layout (responsive desktop only)

# Dashboard Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Build a 10-page React dashboard for monadclaw with cyberpunk terminal aesthetics (neon green / black palette from `tmp/website`) and feature layout from `tmp/openclaw/ui`.

**Architecture:** Vite + React 19 SPA with a 220px sidebar + 56px topbar shell. React Query handles server state (polling the Rust API at `/api/v1`). Zustand holds UI state. Since the Rust backend does not exist yet, all API calls use in-memory mocks that are replaced by real fetches later.

**Tech Stack:** React 19, TypeScript, Vite, React Router v7, TanStack React Query v5, Zustand v5, Recharts, Monaco Editor, Vitest + React Testing Library

**Design reference:**
- Colors / typography / aesthetic → `tmp/website/src/styles/global.css`
- Layout / pages / features → `tmp/openclaw/ui/src/`
- Design spec → `docs/plans/2026-03-10-dashboard-design.md`

---

## Phase 1 — Project Scaffold

### Task 1: Scaffold Vite + React project

**Files:**
- Create: `dashboard/` (entire directory)

**Step 1: Scaffold with Vite**

```bash
cd d:/Documents/perso/monadforge/monadclaw
npm create vite@latest dashboard -- --template react-ts
cd dashboard
npm install
```

**Step 2: Install dependencies**

```bash
npm install react-router-dom @tanstack/react-query zustand recharts
npm install -D vitest @vitest/browser @testing-library/react @testing-library/user-event @testing-library/jest-dom jsdom
```

Monaco editor is large — defer it to the Config page task.

**Step 3: Clean up Vite boilerplate**

Delete: `dashboard/src/assets/react.svg`, `dashboard/public/vite.svg`, `dashboard/src/App.css`, `dashboard/src/index.css` (we replace with our own).
Clear `dashboard/src/App.tsx` and `dashboard/src/main.tsx` to minimal stubs:

```tsx
// src/main.tsx
import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import './styles/base.css'
import App from './App'

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <App />
  </StrictMode>
)
```

```tsx
// src/App.tsx
export default function App() {
  return <div>monadclaw dashboard</div>
}
```

**Step 4: Configure Vite dev proxy**

Edit `dashboard/vite.config.ts`:

```ts
import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

export default defineConfig({
  plugins: [react()],
  server: {
    proxy: {
      '/api': 'http://localhost:3000',
    },
  },
})
```

**Step 5: Configure Vitest**

Edit `dashboard/vite.config.ts` (merge with above):

```ts
import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

export default defineConfig({
  plugins: [react()],
  server: {
    proxy: { '/api': 'http://localhost:3000' },
  },
  test: {
    environment: 'jsdom',
    globals: true,
    setupFiles: ['./src/test/setup.ts'],
  },
})
```

Create `dashboard/src/test/setup.ts`:

```ts
import '@testing-library/jest-dom'
```

**Step 6: Verify dev server starts**

```bash
cd dashboard && npm run dev
```
Expected: server starts at http://localhost:5173, page shows "monadclaw dashboard".

**Step 7: Commit**

```bash
cd ..
git add dashboard/
git commit -m "feat(dashboard): scaffold Vite + React project with dependencies"
```

---

### Task 2: Design tokens and global styles

**Files:**
- Create: `dashboard/src/styles/tokens.css`
- Create: `dashboard/src/styles/base.css`
- Create: `dashboard/src/styles/layout.css`

**Step 1: Write CSS design tokens** (open `tmp/website/src/styles/global.css` for reference)

`dashboard/src/styles/tokens.css`:

```css
:root {
  /* Colors */
  --bg:            #0a0a0a;
  --surface:       #111111;
  --surface-2:     #1a1a1a;
  --border:        #2a2a2a;
  --accent:        #00ff88;
  --accent-dim:    #00cc6a;
  --text:          #e2e8f0;
  --text-muted:    #4a5568;

  /* Semantic */
  --success:       #00ff88;
  --warning:       #f59e0b;
  --error:         #ef4444;
  --info:          #22d3ee;

  /* Typography */
  --font-mono:     'JetBrains Mono', 'Fira Code', monospace;
  --font-sans:     'Inter', system-ui, sans-serif;
  --font-size-xs:  11px;
  --font-size-sm:  13px;
  --font-size-base: 14px;
  --font-size-lg:  16px;
  --font-size-xl:  20px;

  /* Spacing */
  --radius-sm:     4px;
  --radius:        8px;
  --radius-lg:     12px;
  --gap-sm:        8px;
  --gap:           16px;
  --gap-lg:        24px;

  /* Layout */
  --sidebar-w:     220px;
  --sidebar-collapsed-w: 56px;
  --topbar-h:      56px;

  /* Motion */
  --fast:          120ms ease-out;
  --normal:        200ms ease-in-out;
}
```

`dashboard/src/styles/base.css`:

```css
@import './tokens.css';
@import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600&family=JetBrains+Mono:wght@400;500&display=swap');

*, *::before, *::after { box-sizing: border-box; margin: 0; padding: 0; }

html, body, #root {
  height: 100%;
  background: var(--bg);
  color: var(--text);
  font-family: var(--font-sans);
  font-size: var(--font-size-base);
  line-height: 1.55;
  -webkit-font-smoothing: antialiased;
}

a { color: var(--accent); text-decoration: none; }
a:hover { color: var(--accent-dim); text-decoration: underline; }

code, pre, .mono { font-family: var(--font-mono); }

::-webkit-scrollbar { width: 6px; }
::-webkit-scrollbar-track { background: var(--bg); }
::-webkit-scrollbar-thumb { background: var(--border); border-radius: 3px; }
```

`dashboard/src/styles/layout.css`:

```css
.shell {
  display: grid;
  grid-template-rows: var(--topbar-h) 1fr;
  grid-template-columns: var(--sidebar-w) 1fr;
  grid-template-areas:
    "topbar topbar"
    "sidebar content";
  height: 100vh;
  overflow: hidden;
}

.shell.collapsed {
  grid-template-columns: var(--sidebar-collapsed-w) 1fr;
}

.topbar  { grid-area: topbar; }
.sidebar { grid-area: sidebar; overflow-y: auto; }
.content { grid-area: content; overflow-y: auto; padding: var(--gap-lg); }
```

**Step 2: Verify tokens load**

Import `base.css` in `main.tsx` (already done in Task 1). Run `npm run dev`, confirm background is `#0a0a0a`.

**Step 3: Commit**

```bash
git add dashboard/src/styles/
git commit -m "feat(dashboard): add design tokens and global styles"
```

---

## Phase 2 — Shell Layout & Routing

### Task 3: Topbar component

**Files:**
- Create: `dashboard/src/components/layout/Topbar.tsx`
- Create: `dashboard/src/components/layout/Topbar.css`
- Create: `dashboard/src/test/components/Topbar.test.tsx`

**Step 1: Write failing test**

`dashboard/src/test/components/Topbar.test.tsx`:

```tsx
import { render, screen } from '@testing-library/react'
import { Topbar } from '../../components/layout/Topbar'

test('renders monadclaw logo', () => {
  render(<Topbar />)
  expect(screen.getByText('[monadclaw]')).toBeInTheDocument()
})

test('renders status pill', () => {
  render(<Topbar />)
  expect(screen.getByRole('status')).toBeInTheDocument()
})
```

**Step 2: Run test — expect FAIL**

```bash
cd dashboard && npx vitest run src/test/components/Topbar.test.tsx
```
Expected: FAIL — module not found.

**Step 3: Implement Topbar**

`dashboard/src/components/layout/Topbar.css`:

```css
.topbar {
  display: flex;
  align-items: center;
  gap: var(--gap);
  padding: 0 var(--gap-lg);
  background: var(--surface);
  border-bottom: 1px solid var(--border);
  position: sticky;
  top: 0;
  z-index: 100;
}

.topbar-logo {
  font-family: var(--font-mono);
  font-size: var(--font-size-lg);
  color: var(--accent);
  font-weight: 500;
  letter-spacing: -0.02em;
}

.topbar-spacer { flex: 1; }

.status-pill {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 4px 10px;
  border-radius: 99px;
  border: 1px solid var(--border);
  font-size: var(--font-size-xs);
  font-family: var(--font-mono);
  text-transform: uppercase;
  letter-spacing: 0.08em;
}

.status-dot {
  width: 7px;
  height: 7px;
  border-radius: 50%;
  background: var(--text-muted);
}

.status-pill[data-status="online"] .status-dot  { background: var(--success); }
.status-pill[data-status="offline"] .status-dot { background: var(--error); }
.status-pill[data-status="degraded"] .status-dot{ background: var(--warning); }
```

`dashboard/src/components/layout/Topbar.tsx`:

```tsx
import './Topbar.css'

type Status = 'online' | 'offline' | 'degraded' | 'unknown'

interface TopbarProps {
  status?: Status
}

export function Topbar({ status = 'unknown' }: TopbarProps) {
  return (
    <header className="topbar">
      <span className="topbar-logo">[monadclaw]</span>
      <div className="topbar-spacer" />
      <div className="status-pill" data-status={status} role="status">
        <span className="status-dot" />
        <span>{status}</span>
      </div>
    </header>
  )
}
```

**Step 4: Run test — expect PASS**

```bash
npx vitest run src/test/components/Topbar.test.tsx
```

**Step 5: Commit**

```bash
git add src/components/layout/Topbar.tsx src/components/layout/Topbar.css src/test/components/Topbar.test.tsx
git commit -m "feat(dashboard): add Topbar component"
```

---

### Task 4: Sidebar navigation

**Files:**
- Create: `dashboard/src/components/layout/Sidebar.tsx`
- Create: `dashboard/src/components/layout/Sidebar.css`
- Create: `dashboard/src/test/components/Sidebar.test.tsx`

**Step 1: Write failing tests**

`dashboard/src/test/components/Sidebar.test.tsx`:

```tsx
import { render, screen, fireEvent } from '@testing-library/react'
import { MemoryRouter } from 'react-router-dom'
import { Sidebar } from '../../components/layout/Sidebar'

const wrap = (ui: React.ReactElement) =>
  render(<MemoryRouter>{ui}</MemoryRouter>)

test('renders all nav group labels', () => {
  wrap(<Sidebar collapsed={false} onToggle={() => {}} />)
  expect(screen.getByText('MONITOR')).toBeInTheDocument()
  expect(screen.getByText('MANAGE')).toBeInTheDocument()
  expect(screen.getByText('SETTINGS')).toBeInTheDocument()
})

test('renders Overview link', () => {
  wrap(<Sidebar collapsed={false} onToggle={() => {}} />)
  expect(screen.getByRole('link', { name: /overview/i })).toBeInTheDocument()
})

test('collapse toggle fires callback', () => {
  const onToggle = vi.fn()
  wrap(<Sidebar collapsed={false} onToggle={onToggle} />)
  fireEvent.click(screen.getByRole('button', { name: /collapse/i }))
  expect(onToggle).toHaveBeenCalledOnce()
})
```

**Step 2: Run — expect FAIL**

```bash
npx vitest run src/test/components/Sidebar.test.tsx
```

**Step 3: Implement Sidebar**

`dashboard/src/components/layout/Sidebar.css`:

```css
.sidebar {
  background: var(--surface);
  border-right: 1px solid var(--border);
  display: flex;
  flex-direction: column;
  overflow-y: auto;
  overflow-x: hidden;
  transition: width var(--normal);
}

.sidebar-toggle {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  padding: var(--gap-sm) var(--gap);
  border-bottom: 1px solid var(--border);
  background: transparent;
  border-left: none;
  border-top: none;
  border-right: none;
  color: var(--text-muted);
  cursor: pointer;
  font-size: 18px;
  transition: color var(--fast);
}
.sidebar-toggle:hover { color: var(--accent); }

.nav-group { padding: var(--gap-sm) 0; }

.nav-group-label {
  padding: var(--gap-sm) var(--gap);
  font-size: var(--font-size-xs);
  font-family: var(--font-mono);
  text-transform: uppercase;
  letter-spacing: 0.08em;
  color: var(--text-muted);
}

.nav-link {
  display: flex;
  align-items: center;
  gap: var(--gap-sm);
  padding: 8px var(--gap);
  color: var(--text-muted);
  text-decoration: none;
  font-size: var(--font-size-sm);
  transition: color var(--fast), background var(--fast);
  border-left: 2px solid transparent;
}

.nav-link:hover {
  color: var(--text);
  background: var(--surface-2);
  text-decoration: none;
}

.nav-link.active {
  color: var(--accent);
  border-left-color: var(--accent);
  background: rgba(0, 255, 136, 0.06);
}

.nav-icon { width: 16px; text-align: center; flex-shrink: 0; }
```

`dashboard/src/components/layout/Sidebar.tsx`:

```tsx
import { NavLink } from 'react-router-dom'
import './Sidebar.css'

interface NavItem { label: string; path: string; icon: string }
interface NavGroup { label: string; items: NavItem[] }

const NAV: NavGroup[] = [
  {
    label: 'MONITOR',
    items: [
      { label: 'Overview', path: '/', icon: '◈' },
      { label: 'Logs',     path: '/logs', icon: '≡' },
      { label: 'Sessions', path: '/sessions', icon: '◷' },
      { label: 'Usage',    path: '/usage', icon: '◎' },
    ],
  },
  {
    label: 'MANAGE',
    items: [
      { label: 'Agents',   path: '/agents', icon: '◉' },
      { label: 'Channels', path: '/channels', icon: '◈' },
      { label: 'Skills',   path: '/skills', icon: '◆' },
      { label: 'Cron',     path: '/cron', icon: '◷' },
    ],
  },
  {
    label: 'SETTINGS',
    items: [
      { label: 'Config',   path: '/config', icon: '◧' },
    ],
  },
  {
    label: 'INTERACT',
    items: [
      { label: 'Chat',     path: '/chat', icon: '◌' },
    ],
  },
]

interface SidebarProps {
  collapsed: boolean
  onToggle: () => void
}

export function Sidebar({ collapsed, onToggle }: SidebarProps) {
  return (
    <nav className={`sidebar${collapsed ? ' collapsed' : ''}`}>
      <button
        className="sidebar-toggle"
        onClick={onToggle}
        aria-label="collapse sidebar"
      >
        {collapsed ? '›' : '‹'}
      </button>
      {NAV.map(group => (
        <div key={group.label} className="nav-group">
          {!collapsed && (
            <div className="nav-group-label">{group.label}</div>
          )}
          {group.items.map(item => (
            <NavLink
              key={item.path}
              to={item.path}
              end={item.path === '/'}
              className={({ isActive }) =>
                `nav-link${isActive ? ' active' : ''}`
              }
            >
              <span className="nav-icon">{item.icon}</span>
              {!collapsed && <span>{item.label}</span>}
            </NavLink>
          ))}
        </div>
      ))}
    </nav>
  )
}
```

**Step 4: Run — expect PASS**

```bash
npx vitest run src/test/components/Sidebar.test.tsx
```

**Step 5: Commit**

```bash
git add src/components/layout/Sidebar.tsx src/components/layout/Sidebar.css src/test/components/Sidebar.test.tsx
git commit -m "feat(dashboard): add Sidebar navigation component"
```

---

### Task 5: Shell layout + router

**Files:**
- Create: `dashboard/src/components/layout/Shell.tsx`
- Create: `dashboard/src/store/uiStore.ts`
- Modify: `dashboard/src/App.tsx`
- Create: `dashboard/src/pages/` (stub files for all 10 pages)

**Step 1: Create Zustand UI store**

`dashboard/src/store/uiStore.ts`:

```ts
import { create } from 'zustand'
import { persist } from 'zustand/middleware'

interface UiState {
  sidebarCollapsed: boolean
  toggleSidebar: () => void
}

export const useUiStore = create<UiState>()(
  persist(
    set => ({
      sidebarCollapsed: false,
      toggleSidebar: () =>
        set(s => ({ sidebarCollapsed: !s.sidebarCollapsed })),
    }),
    { name: 'monadclaw-ui' }
  )
)
```

**Step 2: Create Shell component**

`dashboard/src/components/layout/Shell.tsx`:

```tsx
import { Outlet } from 'react-router-dom'
import { Topbar } from './Topbar'
import { Sidebar } from './Sidebar'
import { useUiStore } from '../../store/uiStore'
import '../../styles/layout.css'

export function Shell() {
  const { sidebarCollapsed, toggleSidebar } = useUiStore()
  return (
    <div className={`shell${sidebarCollapsed ? ' collapsed' : ''}`}>
      <Topbar />
      <Sidebar collapsed={sidebarCollapsed} onToggle={toggleSidebar} />
      <main className="content">
        <Outlet />
      </main>
    </div>
  )
}
```

**Step 3: Create stub pages**

Create one file per page (same pattern for all 10):

```tsx
// Example: dashboard/src/pages/OverviewPage.tsx
export default function OverviewPage() {
  return <div><h1>Overview</h1></div>
}
```

Repeat for: `ChatPage`, `ChannelsPage`, `AgentsPage`, `ConfigPage`, `SessionsPage`, `LogsPage`, `UsagePage`, `CronPage`, `SkillsPage`.

**Step 4: Wire up router in App.tsx**

`dashboard/src/App.tsx`:

```tsx
import { BrowserRouter, Routes, Route } from 'react-router-dom'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { Shell } from './components/layout/Shell'
import OverviewPage  from './pages/OverviewPage'
import ChatPage      from './pages/ChatPage'
import ChannelsPage  from './pages/ChannelsPage'
import AgentsPage    from './pages/AgentsPage'
import ConfigPage    from './pages/ConfigPage'
import SessionsPage  from './pages/SessionsPage'
import LogsPage      from './pages/LogsPage'
import UsagePage     from './pages/UsagePage'
import CronPage      from './pages/CronPage'
import SkillsPage    from './pages/SkillsPage'

const queryClient = new QueryClient()

export default function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <BrowserRouter>
        <Routes>
          <Route element={<Shell />}>
            <Route index         element={<OverviewPage />} />
            <Route path="chat"   element={<ChatPage />} />
            <Route path="channels" element={<ChannelsPage />} />
            <Route path="agents"   element={<AgentsPage />} />
            <Route path="config"   element={<ConfigPage />} />
            <Route path="sessions" element={<SessionsPage />} />
            <Route path="logs"     element={<LogsPage />} />
            <Route path="usage"    element={<UsagePage />} />
            <Route path="cron"     element={<CronPage />} />
            <Route path="skills"   element={<SkillsPage />} />
          </Route>
        </Routes>
      </BrowserRouter>
    </QueryClientProvider>
  )
}
```

**Step 5: Verify in browser**

```bash
npm run dev
```
Navigate to each route — expect the shell with sidebar and topbar, stub h1 content for each page.

**Step 6: Commit**

```bash
git add src/
git commit -m "feat(dashboard): wire up shell layout and all page routes"
```

---

## Phase 3 — API Layer with Mocks

### Task 6: API client and type definitions

**Files:**
- Create: `dashboard/src/api/client.ts`
- Create: `dashboard/src/types/api.ts`
- Create: `dashboard/src/api/mock.ts`
- Create: `dashboard/src/test/api/client.test.ts`

**Step 1: Define API types**

`dashboard/src/types/api.ts`:

```ts
export interface ApiError {
  error: { code: string; message: string }
}

export interface AgentStatus {
  status: 'online' | 'offline' | 'degraded'
  provider: string
  model: string
  memoryShortTerm: number  // items in window
  memoryLongTerm: number   // stored entries
  uptimeSeconds: number
  messagesToday: number
}

export interface ChatMessage {
  id: string
  role: 'user' | 'assistant' | 'tool'
  content: string
  toolName?: string
  timestamp: string
}

export interface Channel {
  id: string
  type: 'discord' | 'rest' | 'telegram'
  name: string
  enabled: boolean
  config: Record<string, unknown>
}

export interface Session {
  id: string
  channelType: string
  messageCount: number
  provider: string
  startedAt: string
  endedAt?: string
}

export interface LogEntry {
  timestamp: string
  level: 'DEBUG' | 'INFO' | 'WARN' | 'ERROR'
  message: string
  target?: string
}

export interface UsageStat {
  date: string
  provider: string
  model: string
  inputTokens: number
  outputTokens: number
  estimatedCostUsd: number
}

export interface CronJob {
  id: string
  schedule: string       // cron expression
  description: string
  enabled: boolean
  lastRun?: string
  lastStatus?: 'ok' | 'error'
  nextRun: string
}

export interface Skill {
  id: string
  name: string
  version: string
  enabled: boolean
  description: string
  configSchema?: Record<string, unknown>
}
```

**Step 2: Write failing test for client**

`dashboard/src/test/api/client.test.ts`:

```ts
import { describe, test, expect, vi, beforeEach } from 'vitest'
import { apiFetch } from '../../api/client'

describe('apiFetch', () => {
  beforeEach(() => {
    vi.stubGlobal('fetch', vi.fn())
  })

  test('adds Authorization header when token present', async () => {
    const mockFetch = vi.mocked(fetch)
    mockFetch.mockResolvedValueOnce(
      new Response(JSON.stringify({ ok: true }), { status: 200 })
    )

    localStorage.setItem('monadclaw-token', 'test-token')
    await apiFetch('/api/v1/status')

    expect(mockFetch).toHaveBeenCalledWith(
      '/api/v1/status',
      expect.objectContaining({
        headers: expect.objectContaining({
          Authorization: 'Bearer test-token',
        }),
      })
    )
  })

  test('throws ApiError on non-2xx response', async () => {
    vi.mocked(fetch).mockResolvedValueOnce(
      new Response(
        JSON.stringify({ error: { code: 'NOT_FOUND', message: 'not found' } }),
        { status: 404 }
      )
    )

    await expect(apiFetch('/api/v1/missing')).rejects.toMatchObject({
      error: { code: 'NOT_FOUND' },
    })
  })
})
```

**Step 3: Run — expect FAIL**

```bash
npx vitest run src/test/api/client.test.ts
```

**Step 4: Implement client**

`dashboard/src/api/client.ts`:

```ts
import type { ApiError } from '../types/api'

export async function apiFetch<T>(
  path: string,
  options: RequestInit = {}
): Promise<T> {
  const token = localStorage.getItem('monadclaw-token')
  const headers: Record<string, string> = {
    'Content-Type': 'application/json',
    ...(options.headers as Record<string, string>),
  }
  if (token) headers['Authorization'] = `Bearer ${token}`

  const res = await fetch(path, { ...options, headers })

  if (!res.ok) {
    const body: ApiError = await res.json()
    throw body
  }

  return res.json() as Promise<T>
}
```

**Step 5: Create mock data**

`dashboard/src/api/mock.ts` — export typed mock objects matching the types above. Used when `import.meta.env.VITE_USE_MOCK === 'true'` (set this in `.env.development.local`):

```ts
import type {
  AgentStatus, ChatMessage, Channel, Session,
  LogEntry, UsageStat, CronJob, Skill
} from '../types/api'

export const mockStatus: AgentStatus = {
  status: 'online',
  provider: 'openai',
  model: 'gpt-4o',
  memoryShortTerm: 12,
  memoryLongTerm: 347,
  uptimeSeconds: 86400,
  messagesToday: 42,
}

export const mockMessages: ChatMessage[] = [
  { id: '1', role: 'user', content: 'Hello agent', timestamp: new Date().toISOString() },
  { id: '2', role: 'assistant', content: 'Hello! How can I help?', timestamp: new Date().toISOString() },
]

export const mockChannels: Channel[] = [
  { id: 'discord-1', type: 'discord', name: 'Main Guild', enabled: true, config: {} },
  { id: 'rest-1',    type: 'rest',    name: 'REST API',   enabled: true, config: {} },
]

export const mockLogs: LogEntry[] = [
  { timestamp: new Date().toISOString(), level: 'INFO',  message: 'Agent started', target: 'core' },
  { timestamp: new Date().toISOString(), level: 'DEBUG', message: 'Memory loaded: 347 entries', target: 'memory' },
  { timestamp: new Date().toISOString(), level: 'WARN',  message: 'Provider rate limit approaching', target: 'providers' },
]

// Add similar minimal mock arrays for: mockSessions, mockUsage, mockCrons, mockSkills
```

**Step 6: Create API modules** (one per resource, thin wrappers that check mock flag)

`dashboard/src/api/status.ts`:

```ts
import { apiFetch } from './client'
import { mockStatus } from './mock'
import type { AgentStatus } from '../types/api'

const USE_MOCK = import.meta.env.VITE_USE_MOCK === 'true'

export async function fetchStatus(): Promise<AgentStatus> {
  if (USE_MOCK) return mockStatus
  return apiFetch<AgentStatus>('/api/v1/status')
}
```

Repeat the same pattern for `chat.ts`, `channels.ts`, `sessions.ts`, `logs.ts`, `usage.ts`, `cron.ts`, `skills.ts`, `agents.ts`, `config.ts`.

**Step 7: Create `.env.development.local`** (gitignored)

```
VITE_USE_MOCK=true
```

**Step 8: Run tests — expect PASS**

```bash
npx vitest run src/test/api/
```

**Step 9: Commit**

```bash
git add src/api/ src/types/ src/test/api/
git commit -m "feat(dashboard): add API client, types, and mock layer"
```

---

## Phase 4 — Shared UI Components

### Task 7: Card, Badge, StatusPill, Button

**Files:**
- Create: `dashboard/src/components/ui/` (Card, Badge, StatusPill, Button — each with .tsx and .css)

**Step 1: Implement without separate tests** (these are thin presentational components — test them implicitly via page tests)

`dashboard/src/components/ui/Card.tsx`:

```tsx
import './Card.css'
interface CardProps {
  title?: string
  children: React.ReactNode
  className?: string
}
export function Card({ title, children, className = '' }: CardProps) {
  return (
    <div className={`card ${className}`}>
      {title && <div className="card-title">{title}</div>}
      <div className="card-body">{children}</div>
    </div>
  )
}
```

`dashboard/src/components/ui/Card.css`:

```css
.card {
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: var(--radius);
  overflow: hidden;
}
.card-title {
  padding: 12px var(--gap);
  border-bottom: 1px solid var(--border);
  font-family: var(--font-mono);
  font-size: var(--font-size-sm);
  color: var(--text-muted);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}
.card-body { padding: var(--gap); }
```

`dashboard/src/components/ui/Badge.tsx`:

```tsx
import './Badge.css'
type BadgeVariant = 'success' | 'warning' | 'error' | 'info' | 'neutral'
interface BadgeProps { label: string; variant?: BadgeVariant }
export function Badge({ label, variant = 'neutral' }: BadgeProps) {
  return <span className={`badge badge-${variant}`}>{label}</span>
}
```

`dashboard/src/components/ui/Badge.css`:

```css
.badge {
  display: inline-block;
  padding: 2px 8px;
  border-radius: 99px;
  font-size: var(--font-size-xs);
  font-family: var(--font-mono);
  text-transform: uppercase;
  letter-spacing: 0.06em;
  border: 1px solid currentColor;
}
.badge-success { color: var(--success); }
.badge-warning { color: var(--warning); }
.badge-error   { color: var(--error); }
.badge-info    { color: var(--info); }
.badge-neutral { color: var(--text-muted); }
```

`dashboard/src/components/ui/Button.tsx`:

```tsx
import './Button.css'
interface ButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: 'primary' | 'ghost' | 'danger'
}
export function Button({ variant = 'primary', className = '', ...props }: ButtonProps) {
  return (
    <button className={`btn btn-${variant} ${className}`} {...props} />
  )
}
```

`dashboard/src/components/ui/Button.css`:

```css
.btn {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 7px 16px;
  border-radius: var(--radius-sm);
  font-size: var(--font-size-sm);
  font-family: var(--font-mono);
  cursor: pointer;
  transition: background var(--fast), color var(--fast);
  border: 1px solid transparent;
}
.btn-primary  { background: var(--accent); color: #000; border-color: var(--accent); }
.btn-primary:hover { background: var(--accent-dim); border-color: var(--accent-dim); }
.btn-ghost    { background: transparent; color: var(--text-muted); border-color: var(--border); }
.btn-ghost:hover { color: var(--text); border-color: var(--text-muted); }
.btn-danger   { background: transparent; color: var(--error); border-color: var(--error); }
.btn-danger:hover { background: var(--error); color: #fff; }
```

**Step 2: Commit**

```bash
git add src/components/ui/
git commit -m "feat(dashboard): add Card, Badge, Button UI components"
```

---

## Phase 5 — Monitor Pages

### Task 8: Overview page

**Files:**
- Modify: `dashboard/src/pages/OverviewPage.tsx`
- Create: `dashboard/src/hooks/useStatus.ts`
- Create: `dashboard/src/pages/OverviewPage.css`
- Create: `dashboard/src/test/pages/OverviewPage.test.tsx`

**Step 1: Write failing test**

`dashboard/src/test/pages/OverviewPage.test.tsx`:

```tsx
import { render, screen } from '@testing-library/react'
import { QueryClientProvider, QueryClient } from '@tanstack/react-query'
import OverviewPage from '../../pages/OverviewPage'

vi.mock('../../api/status', () => ({
  fetchStatus: vi.fn().mockResolvedValue({
    status: 'online',
    provider: 'openai',
    model: 'gpt-4o',
    memoryShortTerm: 12,
    memoryLongTerm: 347,
    uptimeSeconds: 3600,
    messagesToday: 5,
  }),
}))

test('renders status cards with data', async () => {
  const client = new QueryClient({ defaultOptions: { queries: { retry: false } } })
  render(
    <QueryClientProvider client={client}>
      <OverviewPage />
    </QueryClientProvider>
  )
  expect(await screen.findByText('online')).toBeInTheDocument()
  expect(await screen.findByText('gpt-4o')).toBeInTheDocument()
})
```

**Step 2: Run — expect FAIL**

```bash
npx vitest run src/test/pages/OverviewPage.test.tsx
```

**Step 3: Implement hook and page**

`dashboard/src/hooks/useStatus.ts`:

```ts
import { useQuery } from '@tanstack/react-query'
import { fetchStatus } from '../api/status'

export function useStatus() {
  return useQuery({
    queryKey: ['status'],
    queryFn: fetchStatus,
    refetchInterval: 10_000,
  })
}
```

`dashboard/src/pages/OverviewPage.css`:

```css
.overview-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
  gap: var(--gap);
  margin-bottom: var(--gap-lg);
}
.stat-value {
  font-family: var(--font-mono);
  font-size: var(--font-size-xl);
  color: var(--accent);
  margin-bottom: 4px;
}
.stat-label {
  font-size: var(--font-size-xs);
  color: var(--text-muted);
  text-transform: uppercase;
  letter-spacing: 0.06em;
}
```

`dashboard/src/pages/OverviewPage.tsx`:

```tsx
import { useStatus } from '../hooks/useStatus'
import { Card } from '../components/ui/Card'
import { Badge } from '../components/ui/Badge'
import './OverviewPage.css'

function formatUptime(seconds: number) {
  const h = Math.floor(seconds / 3600)
  const m = Math.floor((seconds % 3600) / 60)
  return `${h}h ${m}m`
}

export default function OverviewPage() {
  const { data, isLoading, error } = useStatus()

  if (isLoading) return <div>Loading...</div>
  if (error || !data) return <div>Failed to load status</div>

  const statusVariant =
    data.status === 'online' ? 'success' :
    data.status === 'degraded' ? 'warning' : 'error'

  return (
    <div>
      <h1 style={{ fontFamily: 'var(--font-mono)', marginBottom: 'var(--gap-lg)' }}>
        Overview
      </h1>
      <div className="overview-grid">
        <Card title="Agent Status">
          <Badge label={data.status} variant={statusVariant} />
        </Card>
        <Card title="Provider">
          <div className="stat-value">{data.model}</div>
          <div className="stat-label">{data.provider}</div>
        </Card>
        <Card title="Memory">
          <div className="stat-value">{data.memoryShortTerm}</div>
          <div className="stat-label">Short-term items</div>
          <div className="stat-value" style={{ marginTop: 8 }}>{data.memoryLongTerm}</div>
          <div className="stat-label">Long-term entries</div>
        </Card>
        <Card title="Messages Today">
          <div className="stat-value">{data.messagesToday}</div>
        </Card>
        <Card title="Uptime">
          <div className="stat-value">{formatUptime(data.uptimeSeconds)}</div>
        </Card>
      </div>
    </div>
  )
}
```

**Step 4: Run — expect PASS**

```bash
npx vitest run src/test/pages/OverviewPage.test.tsx
```

**Step 5: Commit**

```bash
git add src/pages/OverviewPage.tsx src/pages/OverviewPage.css src/hooks/useStatus.ts src/test/pages/OverviewPage.test.tsx
git commit -m "feat(dashboard): implement Overview page with status cards"
```

---

### Task 9: Logs page (real-time stream)

**Files:**
- Modify: `dashboard/src/pages/LogsPage.tsx`
- Create: `dashboard/src/pages/LogsPage.css`
- Create: `dashboard/src/hooks/useLogs.ts`

**Step 1: Implement hook** (polling fallback — SSE when backend ready)

`dashboard/src/hooks/useLogs.ts`:

```ts
import { useQuery } from '@tanstack/react-query'
import { fetchLogs } from '../api/logs'
import { useState } from 'react'
import type { LogEntry } from '../types/api'

type Level = 'ALL' | 'DEBUG' | 'INFO' | 'WARN' | 'ERROR'

export function useLogs() {
  const [level, setLevel] = useState<Level>('ALL')
  const [search, setSearch] = useState('')

  const { data = [] } = useQuery<LogEntry[]>({
    queryKey: ['logs'],
    queryFn: fetchLogs,
    refetchInterval: 3_000,
  })

  const filtered = data.filter(e =>
    (level === 'ALL' || e.level === level) &&
    (search === '' || e.message.toLowerCase().includes(search.toLowerCase()))
  )

  return { logs: filtered, level, setLevel, search, setSearch }
}
```

**Step 2: Implement page**

`dashboard/src/pages/LogsPage.css`:

```css
.logs-toolbar {
  display: flex;
  gap: var(--gap-sm);
  margin-bottom: var(--gap);
  align-items: center;
}
.logs-search {
  flex: 1;
  background: var(--surface);
  border: 1px solid var(--border);
  color: var(--text);
  padding: 6px 12px;
  border-radius: var(--radius-sm);
  font-family: var(--font-mono);
  font-size: var(--font-size-sm);
}
.logs-search:focus { outline: none; border-color: var(--accent); }

.log-list {
  font-family: var(--font-mono);
  font-size: var(--font-size-sm);
  display: flex;
  flex-direction: column;
  gap: 2px;
}
.log-entry {
  display: grid;
  grid-template-columns: 180px 60px 1fr;
  gap: var(--gap-sm);
  padding: 4px 8px;
  border-radius: var(--radius-sm);
}
.log-entry:hover { background: var(--surface); }
.log-entry[data-level="DEBUG"] { color: var(--text-muted); }
.log-entry[data-level="INFO"]  { color: var(--text); }
.log-entry[data-level="WARN"]  { color: var(--warning); }
.log-entry[data-level="ERROR"] { color: var(--error); }
.log-time { color: var(--text-muted); }
.log-level { font-weight: 500; }
```

`dashboard/src/pages/LogsPage.tsx`:

```tsx
import { useLogs } from '../hooks/useLogs'
import { Button } from '../components/ui/Button'
import './LogsPage.css'

const LEVELS = ['ALL', 'DEBUG', 'INFO', 'WARN', 'ERROR'] as const

export default function LogsPage() {
  const { logs, level, setLevel, search, setSearch } = useLogs()

  return (
    <div>
      <h1 style={{ fontFamily: 'var(--font-mono)', marginBottom: 'var(--gap-lg)' }}>Logs</h1>
      <div className="logs-toolbar">
        {LEVELS.map(l => (
          <Button
            key={l}
            variant={level === l ? 'primary' : 'ghost'}
            onClick={() => setLevel(l)}
          >
            {l}
          </Button>
        ))}
        <input
          className="logs-search"
          placeholder="Search logs..."
          value={search}
          onChange={e => setSearch(e.target.value)}
        />
      </div>
      <div className="log-list">
        {logs.map((entry, i) => (
          <div key={i} className="log-entry" data-level={entry.level}>
            <span className="log-time">{new Date(entry.timestamp).toLocaleTimeString()}</span>
            <span className="log-level">{entry.level}</span>
            <span className="log-message">{entry.message}</span>
          </div>
        ))}
        {logs.length === 0 && (
          <div style={{ color: 'var(--text-muted)', padding: 'var(--gap)' }}>No logs match filter.</div>
        )}
      </div>
    </div>
  )
}
```

**Step 3: Commit**

```bash
git add src/pages/LogsPage.tsx src/pages/LogsPage.css src/hooks/useLogs.ts
git commit -m "feat(dashboard): implement Logs page with level filter and search"
```

---

### Task 10: Sessions page

**Files:**
- Modify: `dashboard/src/pages/SessionsPage.tsx`

**Step 1: Implement**

`dashboard/src/pages/SessionsPage.tsx`:

```tsx
import { useQuery } from '@tanstack/react-query'
import { fetchSessions } from '../api/sessions'
import { Badge } from '../components/ui/Badge'
import type { Session } from '../types/api'

export default function SessionsPage() {
  const { data: sessions = [], isLoading } = useQuery<Session[]>({
    queryKey: ['sessions'],
    queryFn: fetchSessions,
  })

  if (isLoading) return <div>Loading...</div>

  return (
    <div>
      <h1 style={{ fontFamily: 'var(--font-mono)', marginBottom: 'var(--gap-lg)' }}>Sessions</h1>
      <table style={{ width: '100%', borderCollapse: 'collapse', fontFamily: 'var(--font-mono)', fontSize: 'var(--font-size-sm)' }}>
        <thead>
          <tr style={{ borderBottom: '1px solid var(--border)', color: 'var(--text-muted)', textAlign: 'left' }}>
            <th style={{ padding: '8px 12px' }}>Started</th>
            <th style={{ padding: '8px 12px' }}>Channel</th>
            <th style={{ padding: '8px 12px' }}>Provider</th>
            <th style={{ padding: '8px 12px' }}>Messages</th>
          </tr>
        </thead>
        <tbody>
          {sessions.map(s => (
            <tr key={s.id} style={{ borderBottom: '1px solid var(--border)' }}>
              <td style={{ padding: '8px 12px' }}>{new Date(s.startedAt).toLocaleString()}</td>
              <td style={{ padding: '8px 12px' }}><Badge label={s.channelType} variant="info" /></td>
              <td style={{ padding: '8px 12px' }}>{s.provider}</td>
              <td style={{ padding: '8px 12px' }}>{s.messageCount}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  )
}
```

**Step 2: Commit**

```bash
git add src/pages/SessionsPage.tsx
git commit -m "feat(dashboard): implement Sessions page"
```

---

### Task 11: Usage page

**Files:**
- Modify: `dashboard/src/pages/UsagePage.tsx`

**Step 1: Install recharts** (already in package.json from Task 1)

**Step 2: Implement**

`dashboard/src/pages/UsagePage.tsx`:

```tsx
import { useQuery } from '@tanstack/react-query'
import { fetchUsage } from '../api/usage'
import { BarChart, Bar, XAxis, YAxis, Tooltip, ResponsiveContainer } from 'recharts'
import { Card } from '../components/ui/Card'
import type { UsageStat } from '../types/api'

export default function UsagePage() {
  const { data: stats = [], isLoading } = useQuery<UsageStat[]>({
    queryKey: ['usage'],
    queryFn: fetchUsage,
  })

  if (isLoading) return <div>Loading...</div>

  const totalCost = stats.reduce((sum, s) => sum + s.estimatedCostUsd, 0)
  const totalTokens = stats.reduce((sum, s) => sum + s.inputTokens + s.outputTokens, 0)

  return (
    <div>
      <h1 style={{ fontFamily: 'var(--font-mono)', marginBottom: 'var(--gap-lg)' }}>Usage</h1>
      <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: 'var(--gap)', marginBottom: 'var(--gap-lg)' }}>
        <Card title="Total Tokens">
          <div style={{ fontFamily: 'var(--font-mono)', fontSize: 'var(--font-size-xl)', color: 'var(--accent)' }}>
            {totalTokens.toLocaleString()}
          </div>
        </Card>
        <Card title="Estimated Cost">
          <div style={{ fontFamily: 'var(--font-mono)', fontSize: 'var(--font-size-xl)', color: 'var(--accent)' }}>
            ${totalCost.toFixed(4)}
          </div>
        </Card>
      </div>
      <Card title="Tokens by Day">
        <ResponsiveContainer width="100%" height={250}>
          <BarChart data={stats}>
            <XAxis dataKey="date" stroke="var(--text-muted)" tick={{ fontSize: 11 }} />
            <YAxis stroke="var(--text-muted)" tick={{ fontSize: 11 }} />
            <Tooltip
              contentStyle={{ background: 'var(--surface-2)', border: '1px solid var(--border)', borderRadius: 8 }}
              labelStyle={{ color: 'var(--text)' }}
            />
            <Bar dataKey="inputTokens" stackId="a" fill="var(--info)" name="Input" />
            <Bar dataKey="outputTokens" stackId="a" fill="var(--accent)" name="Output" />
          </BarChart>
        </ResponsiveContainer>
      </Card>
    </div>
  )
}
```

**Step 3: Commit**

```bash
git add src/pages/UsagePage.tsx
git commit -m "feat(dashboard): implement Usage page with token charts"
```

---

## Phase 6 — Manage Pages

### Task 12: Channels page

**Files:**
- Modify: `dashboard/src/pages/ChannelsPage.tsx`

**Step 1: Implement**

`dashboard/src/pages/ChannelsPage.tsx`:

```tsx
import { useQuery } from '@tanstack/react-query'
import { fetchChannels } from '../api/channels'
import { Card } from '../components/ui/Card'
import { Badge } from '../components/ui/Badge'
import { Button } from '../components/ui/Button'
import type { Channel } from '../types/api'

export default function ChannelsPage() {
  const { data: channels = [], isLoading } = useQuery<Channel[]>({
    queryKey: ['channels'],
    queryFn: fetchChannels,
  })

  if (isLoading) return <div>Loading...</div>

  return (
    <div>
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: 'var(--gap-lg)' }}>
        <h1 style={{ fontFamily: 'var(--font-mono)' }}>Channels</h1>
      </div>
      <div style={{ display: 'flex', flexDirection: 'column', gap: 'var(--gap-sm)' }}>
        {channels.map(ch => (
          <Card key={ch.id}>
            <div style={{ display: 'flex', alignItems: 'center', gap: 'var(--gap)' }}>
              <Badge label={ch.type} variant="info" />
              <span style={{ fontFamily: 'var(--font-mono)', flex: 1 }}>{ch.name}</span>
              <Badge label={ch.enabled ? 'enabled' : 'disabled'} variant={ch.enabled ? 'success' : 'neutral'} />
              <Button variant="ghost">Configure</Button>
            </div>
          </Card>
        ))}
      </div>
    </div>
  )
}
```

**Step 2: Commit**

```bash
git add src/pages/ChannelsPage.tsx
git commit -m "feat(dashboard): implement Channels page"
```

---

### Task 13: Cron page

**Files:**
- Modify: `dashboard/src/pages/CronPage.tsx`

**Step 1: Implement**

`dashboard/src/pages/CronPage.tsx`:

```tsx
import { useQuery } from '@tanstack/react-query'
import { fetchCron } from '../api/cron'
import { Badge } from '../components/ui/Badge'
import { Button } from '../components/ui/Button'
import type { CronJob } from '../types/api'

export default function CronPage() {
  const { data: jobs = [], isLoading } = useQuery<CronJob[]>({
    queryKey: ['cron'],
    queryFn: fetchCron,
  })

  if (isLoading) return <div>Loading...</div>

  return (
    <div>
      <h1 style={{ fontFamily: 'var(--font-mono)', marginBottom: 'var(--gap-lg)' }}>Cron Jobs</h1>
      <table style={{ width: '100%', borderCollapse: 'collapse', fontFamily: 'var(--font-mono)', fontSize: 'var(--font-size-sm)' }}>
        <thead>
          <tr style={{ borderBottom: '1px solid var(--border)', color: 'var(--text-muted)', textAlign: 'left' }}>
            <th style={{ padding: '8px 12px' }}>Schedule</th>
            <th style={{ padding: '8px 12px' }}>Description</th>
            <th style={{ padding: '8px 12px' }}>Next Run</th>
            <th style={{ padding: '8px 12px' }}>Last Status</th>
            <th style={{ padding: '8px 12px' }}>Enabled</th>
          </tr>
        </thead>
        <tbody>
          {jobs.map(job => (
            <tr key={job.id} style={{ borderBottom: '1px solid var(--border)' }}>
              <td style={{ padding: '8px 12px', color: 'var(--accent)' }}>{job.schedule}</td>
              <td style={{ padding: '8px 12px' }}>{job.description}</td>
              <td style={{ padding: '8px 12px', color: 'var(--text-muted)' }}>{new Date(job.nextRun).toLocaleString()}</td>
              <td style={{ padding: '8px 12px' }}>
                {job.lastStatus && (
                  <Badge label={job.lastStatus} variant={job.lastStatus === 'ok' ? 'success' : 'error'} />
                )}
              </td>
              <td style={{ padding: '8px 12px' }}>
                <Button variant={job.enabled ? 'primary' : 'ghost'} style={{ padding: '3px 10px' }}>
                  {job.enabled ? 'On' : 'Off'}
                </Button>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  )
}
```

**Step 2: Commit**

```bash
git add src/pages/CronPage.tsx
git commit -m "feat(dashboard): implement Cron page"
```

---

### Task 14: Skills and Agents pages

**Files:**
- Modify: `dashboard/src/pages/SkillsPage.tsx`
- Modify: `dashboard/src/pages/AgentsPage.tsx`

**Step 1: Implement SkillsPage** (list + enable toggle)

`dashboard/src/pages/SkillsPage.tsx`:

```tsx
import { useQuery } from '@tanstack/react-query'
import { fetchSkills } from '../api/skills'
import { Card } from '../components/ui/Card'
import { Badge } from '../components/ui/Badge'
import { Button } from '../components/ui/Button'
import type { Skill } from '../types/api'

export default function SkillsPage() {
  const { data: skills = [], isLoading } = useQuery<Skill[]>({
    queryKey: ['skills'],
    queryFn: fetchSkills,
  })

  if (isLoading) return <div>Loading...</div>

  return (
    <div>
      <h1 style={{ fontFamily: 'var(--font-mono)', marginBottom: 'var(--gap-lg)' }}>Skills</h1>
      <div style={{ display: 'flex', flexDirection: 'column', gap: 'var(--gap-sm)' }}>
        {skills.map(skill => (
          <Card key={skill.id}>
            <div style={{ display: 'flex', alignItems: 'center', gap: 'var(--gap)' }}>
              <div style={{ flex: 1 }}>
                <div style={{ fontFamily: 'var(--font-mono)', marginBottom: 4 }}>
                  {skill.name} <span style={{ color: 'var(--text-muted)', fontSize: 'var(--font-size-xs)' }}>v{skill.version}</span>
                </div>
                <div style={{ fontSize: 'var(--font-size-sm)', color: 'var(--text-muted)' }}>{skill.description}</div>
              </div>
              <Badge label={skill.enabled ? 'enabled' : 'disabled'} variant={skill.enabled ? 'success' : 'neutral'} />
              <Button variant="ghost">Configure</Button>
            </div>
          </Card>
        ))}
      </div>
    </div>
  )
}
```

**Step 2: Implement AgentsPage** (agent detail — single agent in v1)

`dashboard/src/pages/AgentsPage.tsx`:

```tsx
import { useState } from 'react'
import { Button } from '../components/ui/Button'
import { Card } from '../components/ui/Card'

type Tab = 'tools' | 'files' | 'config' | 'channels'

export default function AgentsPage() {
  const [activeTab, setActiveTab] = useState<Tab>('tools')

  return (
    <div>
      <h1 style={{ fontFamily: 'var(--font-mono)', marginBottom: 'var(--gap-lg)' }}>Agent</h1>
      <div style={{ display: 'flex', gap: 'var(--gap-sm)', marginBottom: 'var(--gap)' }}>
        {(['tools', 'files', 'config', 'channels'] as Tab[]).map(t => (
          <Button
            key={t}
            variant={activeTab === t ? 'primary' : 'ghost'}
            onClick={() => setActiveTab(t)}
          >
            {t}
          </Button>
        ))}
      </div>
      <Card title={activeTab.toUpperCase()}>
        <div style={{ color: 'var(--text-muted)', fontFamily: 'var(--font-mono)' }}>
          {activeTab} tab — connect to /api/v1/agents endpoint
        </div>
      </Card>
    </div>
  )
}
```

**Step 3: Commit**

```bash
git add src/pages/SkillsPage.tsx src/pages/AgentsPage.tsx
git commit -m "feat(dashboard): implement Skills and Agents pages"
```

---

## Phase 7 — Config & Chat

### Task 15: Config page with JSON editor

**Files:**
- Modify: `dashboard/src/pages/ConfigPage.tsx`

**Step 1: Install Monaco Editor**

```bash
npm install @monaco-editor/react
```

**Step 2: Implement**

`dashboard/src/pages/ConfigPage.tsx`:

```tsx
import { useState } from 'react'
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'
import Editor from '@monaco-editor/react'
import { fetchConfig, updateConfig } from '../api/config'
import { Button } from '../components/ui/Button'

export default function ConfigPage() {
  const qc = useQueryClient()
  const { data, isLoading } = useQuery({
    queryKey: ['config'],
    queryFn: fetchConfig,
  })
  const [draft, setDraft] = useState<string | undefined>()
  const [error, setError] = useState('')

  const mutation = useMutation({
    mutationFn: (value: string) => {
      const parsed = JSON.parse(value)
      return updateConfig(parsed)
    },
    onSuccess: () => {
      qc.invalidateQueries({ queryKey: ['config'] })
      setDraft(undefined)
      setError('')
    },
    onError: (e: unknown) => {
      setError(e instanceof Error ? e.message : 'Save failed')
    },
  })

  if (isLoading) return <div>Loading...</div>

  const value = draft ?? JSON.stringify(data, null, 2)

  return (
    <div>
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: 'var(--gap-lg)' }}>
        <h1 style={{ fontFamily: 'var(--font-mono)' }}>Config</h1>
        <div style={{ display: 'flex', gap: 'var(--gap-sm)' }}>
          <Button variant="ghost" onClick={() => { setDraft(undefined); setError('') }}>Discard</Button>
          <Button
            variant="primary"
            disabled={mutation.isPending}
            onClick={() => mutation.mutate(value)}
          >
            {mutation.isPending ? 'Saving…' : 'Save'}
          </Button>
        </div>
      </div>
      {error && (
        <div style={{ color: 'var(--error)', fontFamily: 'var(--font-mono)', marginBottom: 'var(--gap)', fontSize: 'var(--font-size-sm)' }}>
          {error}
        </div>
      )}
      <div style={{ border: '1px solid var(--border)', borderRadius: 'var(--radius)', overflow: 'hidden' }}>
        <Editor
          height="70vh"
          language="json"
          theme="vs-dark"
          value={value}
          onChange={v => setDraft(v)}
          options={{
            fontSize: 13,
            fontFamily: 'JetBrains Mono, monospace',
            minimap: { enabled: false },
            scrollBeyondLastLine: false,
            tabSize: 2,
          }}
        />
      </div>
    </div>
  )
}
```

**Step 3: Commit**

```bash
git add src/pages/ConfigPage.tsx package.json package-lock.json
git commit -m "feat(dashboard): implement Config page with Monaco JSON editor"
```

---

### Task 16: Chat page

**Files:**
- Modify: `dashboard/src/pages/ChatPage.tsx`
- Create: `dashboard/src/pages/ChatPage.css`
- Create: `dashboard/src/hooks/useChat.ts`
- Create: `dashboard/src/test/pages/ChatPage.test.tsx`

**Step 1: Write failing test**

`dashboard/src/test/pages/ChatPage.test.tsx`:

```tsx
import { render, screen, fireEvent } from '@testing-library/react'
import { QueryClientProvider, QueryClient } from '@tanstack/react-query'
import ChatPage from '../../pages/ChatPage'

vi.mock('../../api/chat', () => ({
  fetchHistory: vi.fn().mockResolvedValue([]),
  sendMessage:  vi.fn().mockResolvedValue({ id: '1', role: 'assistant', content: 'Hi!', timestamp: new Date().toISOString() }),
}))

const wrap = (ui: React.ReactElement) =>
  render(<QueryClientProvider client={new QueryClient()}>{ui}</QueryClientProvider>)

test('renders message input', async () => {
  wrap(<ChatPage />)
  expect(await screen.findByPlaceholderText(/message/i)).toBeInTheDocument()
})

test('send button is present', async () => {
  wrap(<ChatPage />)
  expect(await screen.findByRole('button', { name: /send/i })).toBeInTheDocument()
})
```

**Step 2: Run — expect FAIL**

```bash
npx vitest run src/test/pages/ChatPage.test.tsx
```

**Step 3: Implement hook and page**

`dashboard/src/hooks/useChat.ts`:

```ts
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'
import { fetchHistory, sendMessage } from '../api/chat'
import type { ChatMessage } from '../types/api'

export function useChat() {
  const qc = useQueryClient()
  const { data: messages = [] } = useQuery<ChatMessage[]>({
    queryKey: ['chat-history'],
    queryFn: fetchHistory,
  })
  const mutation = useMutation({
    mutationFn: (content: string) => sendMessage(content),
    onSuccess: msg => {
      qc.setQueryData<ChatMessage[]>(['chat-history'], prev => [
        ...(prev ?? []),
        msg,
      ])
    },
  })
  return { messages, send: mutation.mutateAsync, isPending: mutation.isPending }
}
```

`dashboard/src/pages/ChatPage.css`:

```css
.chat-shell {
  display: grid;
  grid-template-rows: 1fr auto;
  height: calc(100vh - var(--topbar-h) - 48px);
  max-height: 800px;
}
.chat-messages {
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: var(--gap-sm);
  padding-bottom: var(--gap);
}
.chat-bubble {
  max-width: 80%;
  padding: 10px 14px;
  border-radius: var(--radius);
  font-size: var(--font-size-sm);
  line-height: 1.6;
}
.chat-bubble.user {
  align-self: flex-end;
  background: var(--surface-2);
  border: 1px solid var(--border);
}
.chat-bubble.assistant {
  align-self: flex-start;
  background: rgba(0, 255, 136, 0.06);
  border: 1px solid rgba(0, 255, 136, 0.2);
}
.chat-input-row {
  display: flex;
  gap: var(--gap-sm);
  padding-top: var(--gap);
  border-top: 1px solid var(--border);
}
.chat-input {
  flex: 1;
  background: var(--surface);
  border: 1px solid var(--border);
  color: var(--text);
  padding: 10px 14px;
  border-radius: var(--radius-sm);
  font-family: var(--font-mono);
  font-size: var(--font-size-sm);
  resize: none;
}
.chat-input:focus { outline: none; border-color: var(--accent); }

@keyframes blink { 50% { opacity: 0; } }
.cursor { animation: blink 1s step-end infinite; color: var(--accent); }
```

`dashboard/src/pages/ChatPage.tsx`:

```tsx
import { useState, useRef, useEffect, KeyboardEvent } from 'react'
import { useChat } from '../hooks/useChat'
import { Button } from '../components/ui/Button'
import './ChatPage.css'

export default function ChatPage() {
  const { messages, send, isPending } = useChat()
  const [input, setInput] = useState('')
  const bottomRef = useRef<HTMLDivElement>(null)

  useEffect(() => {
    bottomRef.current?.scrollIntoView({ behavior: 'smooth' })
  }, [messages])

  const handleSend = async () => {
    const text = input.trim()
    if (!text || isPending) return
    setInput('')
    await send(text)
  }

  const handleKeyDown = (e: KeyboardEvent<HTMLTextAreaElement>) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault()
      handleSend()
    }
  }

  return (
    <div className="chat-shell">
      <div className="chat-messages">
        {messages.map(msg => (
          <div key={msg.id} className={`chat-bubble ${msg.role}`}>
            {msg.content}
          </div>
        ))}
        {isPending && (
          <div className="chat-bubble assistant">
            <span className="cursor">█</span>
          </div>
        )}
        <div ref={bottomRef} />
      </div>
      <div className="chat-input-row">
        <textarea
          className="chat-input"
          rows={2}
          placeholder="Message agent… (Enter to send, Shift+Enter for newline)"
          value={input}
          onChange={e => setInput(e.target.value)}
          onKeyDown={handleKeyDown}
        />
        <Button variant="primary" onClick={handleSend} disabled={isPending}>
          Send
        </Button>
      </div>
    </div>
  )
}
```

**Step 4: Run — expect PASS**

```bash
npx vitest run src/test/pages/ChatPage.test.tsx
```

**Step 5: Commit**

```bash
git add src/pages/ChatPage.tsx src/pages/ChatPage.css src/hooks/useChat.ts src/test/pages/ChatPage.test.tsx
git commit -m "feat(dashboard): implement Chat page with terminal blink cursor"
```

---

## Phase 8 — Polish & Typecheck

### Task 17: Wire Topbar to live status and run full typecheck

**Files:**
- Modify: `dashboard/src/components/layout/Topbar.tsx`
- Modify: `dashboard/src/App.tsx`

**Step 1: Pass live status to Topbar**

In `Shell.tsx`, import `useStatus` and pass `data?.status` to `<Topbar status={data?.status} />`.

**Step 2: Run typecheck**

```bash
cd dashboard && npx tsc --noEmit
```

Fix any type errors found.

**Step 3: Run all tests**

```bash
npx vitest run
```
Expected: all PASS.

**Step 4: Final commit**

```bash
git add src/
git commit -m "feat(dashboard): wire live status to Topbar, fix types, all tests green"
```

---

## Done

At this point the dashboard is:
- Fully scaffolded with Vite + React 19
- 10 pages wired up with mock API data
- Cyberpunk neon-green aesthetic from `tmp/website`
- Layout and feature structure from `tmp/openclaw/ui`
- Ready to swap mocks for real Rust API endpoints (remove `VITE_USE_MOCK=true`)

**Next:** Connect to real Rust API — implement `/api/v1/*` endpoints in the backend.

# Frontend Conventions (React Dashboard)

## Stack

- React 19 + TypeScript
- Build tool: Vite
- Styling: TBD (Tailwind CSS preferred)
- State management: Zustand (local UI state) + React Query (server state)
- Routing: React Router v7

## Dev Commands

```bash
cd dashboard
npm install        # install dependencies
npm run dev        # start dev server (http://localhost:5173)
npm run build      # production build → dashboard/dist/
npm run preview    # preview production build locally
npm run typecheck  # tsc --noEmit
npm run lint       # eslint
```

## Vite Configuration

- Entry point: `dashboard/index.html`
- Dev proxy: `/api` → `http://localhost:3000` (Rust API server)
- Output dir: `dashboard/dist/`

```ts
// dashboard/vite.config.ts (reference)
export default defineConfig({
  plugins: [react()],
  server: {
    proxy: {
      '/api': 'http://localhost:3000',
    },
  },
})
```

## Structure

```
dashboard/
├── index.html
├── vite.config.ts
├── tsconfig.json
├── package.json
└── src/
    ├── main.tsx          # app entry point
    ├── App.tsx           # router setup
    ├── components/       # reusable UI components
    ├── pages/            # route-level components
    │   ├── StatusPage.tsx
    │   ├── ChatPage.tsx
    │   └── ConfigPage.tsx
    ├── api/              # API client functions (fetch wrappers)
    ├── hooks/            # custom React hooks
    ├── store/            # Zustand stores
    └── types/            # shared TypeScript types (mirror API shapes)
```

## Conventions

- Components in PascalCase, files match component name
- API calls isolated in `src/api/`, never inline in components
- No `any` in TypeScript; prefer explicit types
- Shared types mirrored from API response shapes
- `src/api/` functions return typed promises; errors thrown as `ApiError`

## Pages (Planned)

- `/` — Status dashboard (agent health, active providers, memory stats)
- `/chat` — Direct chat interface
- `/config` — Configuration editor

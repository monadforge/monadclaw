# Frontend Conventions (React Dashboard)

## Stack

- React + TypeScript
- Build tool: Vite
- Styling: TBD
- State management: TBD (prefer simple: Zustand or React Query)

## Structure

```
dashboard/
├── src/
│   ├── components/   # Reusable UI components
│   ├── pages/        # Route-level components
│   ├── api/          # API client functions
│   ├── hooks/        # Custom React hooks
│   └── types/        # Shared TypeScript types
└── public/
```

## Conventions

- Components in PascalCase, files match component name
- API calls isolated in `src/api/`, never inline in components
- No `any` in TypeScript; prefer explicit types
- Shared types mirrored from API response shapes

## Pages (Planned)

- `/` — Status dashboard (agent health, active providers)
- `/chat` — Direct chat interface
- `/config` — Configuration editor

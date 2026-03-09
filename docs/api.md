# API Design

## Base

- Base path: `/api/v1`
- Format: JSON
- Auth: Bearer token in `Authorization` header

## Error Format

```json
{
  "error": {
    "code": "NOT_FOUND",
    "message": "Human-readable description"
  }
}
```

## Endpoints (Planned)

### Status
- `GET /api/v1/status` — agent health, active providers, memory stats

### Chat
- `POST /api/v1/chat` — send a message, receive a response
- `GET /api/v1/chat/history` — conversation history

### Configuration
- `GET /api/v1/config` — current configuration (sensitive fields redacted)
- `PATCH /api/v1/config` — update configuration at runtime

### Providers
- `GET /api/v1/providers` — list available and active providers

## Conventions

- Use nouns for resources, not verbs (`/messages` not `/sendMessage`)
- `GET` is always read-only and idempotent
- Return `404` for unknown resources, `422` for validation errors, `401` for auth failures
- Pagination via `?page=` and `?limit=` query params

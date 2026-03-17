# Auth Policy

## Overview

Dashboard access uses a three-tier model based on connection origin and whether a password is configured.

| Origin | Password set? | Result |
|--------|--------------|--------|
| Local (loopback) | No | Allow — no credentials required |
| Local (loopback) | Yes | Require valid Bearer token |
| Remote | No | 403 Forbidden |
| Remote | Yes | Require valid Bearer token |

## Configuration

Set `dashboard_password` in `config.toml`:

```toml
dashboard_password = "your-secret-password"
```

If the field is absent, local access is passwordless and remote access is blocked.

## Login Flow

1. Frontend calls `GET /api/v1/auth/status` → `{"protected": true/false}`
2. If `protected: true` and no token in localStorage → redirect to `/login`
3. User submits password → `POST /api/v1/auth/login` → `{"token": "..."}`
4. Token stored in `localStorage` as `monadclaw-token`
5. All subsequent API calls send `Authorization: Bearer <token>`

## Endpoints

| Method | Path | Auth required | Description |
|--------|------|--------------|-------------|
| `GET` | `/api/v1/auth/status` | No | Returns `{"protected": bool}` |
| `POST` | `/api/v1/auth/login` | No | Validates password, returns token |
| `*` | `/api/v1/*` | Yes (per policy above) | Protected routes |

## Notes

- The token equals the configured password. There is no session expiry.
- To log out, clear `monadclaw-token` from browser localStorage.
- Restart the server after changing `dashboard_password`.

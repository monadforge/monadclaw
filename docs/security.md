# Security

## Secrets Management

- API keys and tokens are never hardcoded or committed
- Loaded exclusively from environment variables or a secrets file outside the repo
- `.env` files are in `.gitignore`

## API Security

- All dashboard API endpoints require authentication
- Tokens have configurable expiry
- Rate limiting on chat endpoints

## LLM Provider Keys

- One key per provider, stored in config
- Keys are redacted in logs and API responses

## Discord Bot

- Bot token stored as environment variable only
- Restrict bot permissions to minimum required

## Checklist Before Commit

- [ ] No secrets in code or config files
- [ ] No `println!` of sensitive data
- [ ] Dependencies reviewed for known CVEs (`cargo audit`)

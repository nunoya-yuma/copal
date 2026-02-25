# README

## Environment

### Ollama install

```shell
curl -fsSL https://ollama.com/install.sh | sh
```

### Environment Variables

Copy `.env.example` to `.env` and set your API keys:

```shell
cp .env.example .env
```

| Variable | Description | Required |
|----------|-------------|----------|
| `COPAL_API_TOKEN` | Bearer token for API authentication (web mode) | Yes (web mode) |
| `TAVILY_API_KEY` | API key for [Tavily](https://tavily.com/) web search | Yes (for web search) |
| `LLM_PROVIDER` | LLM provider (`ollama` / `openai` / `gemini`) | No (default: `ollama`) |
| `OPENAI_API_KEY` | OpenAI API key | Yes (if using OpenAI) |
| `GEMINI_API_KEY` | Gemini API key | Yes (if using Gemini) |

Get your Tavily API key at: https://app.tavily.com/

## Usage

### CLI mode (interactive REPL)

```shell
cargo run
```

Features:
- Command history (up/down arrow keys)
- History persisted to `.copal_history`
- `exit` or `quit` to end session
- `Ctrl+C` to cancel input, `Ctrl+D` to exit

### Web server mode

```shell
cargo run --no-default-features --features web
```

Starts the API server at `http://localhost:3000`. Requires `COPAL_API_TOKEN` to be set in `.env`.
To use with the frontend, also run `npm run dev` in the `frontend/` directory.

### Format

```shell
cargo fmt
```

### Lint

```shell
cargo clippy
```

## Test

### Unit test

```shell
cargo test
```

### E2E test

```shell
cargo test -- --ignored
```

## Frontend

### Setup

```shell
cd frontend
npm ci
```

### Development server

```shell
cd frontend
npm run dev
```

Opens at `http://localhost:5173`. API requests are proxied to the Rust backend at `localhost:3000`.

### Typecheck

```shell
cd frontend
npm run typecheck
```

### Test

```shell
# Watch mode (re-runs on file save, for development)
npm run test

# Run once (for CI or manual verification)
npm run test:ci
```

### Build

```shell
cd frontend
npm run build
```

Outputs optimized static files to `frontend/dist/`, which the Rust backend serves in production.

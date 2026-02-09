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
| `TAVILY_API_KEY` | API key for [Tavily](https://tavily.com/) web search | Yes (for web search) |

Get your Tavily API key at: https://app.tavily.com/

## Usage

### One-shot mode

```shell
cargo run -- "your query"
# e.g.)
cargo run -- "What are the new features in Rust 1.84?"
```

### Interactive mode

```shell
cargo run -- -i
```

Features:
- Command history (↑↓ arrow keys)
- History persisted to `.copal_history`
- `exit` or `quit` to end session
- `Ctrl+C` to cancel input, `Ctrl+D` to exit

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

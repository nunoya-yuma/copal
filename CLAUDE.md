# Copal - パーソナルリサーチエージェント

## プロジェクト概要
Rust製のLLMを活用した個人向け調査・情報整理アシスタント。複数の情報源からデータを収集・統合し、ユーザーの文脈に合わせた深い洞察を提供する。

**コンセプト**: Co（一緒に）+ Pal（友達）= いつも一緒にいる相棒。琥珀の前段階の樹脂という意味もある。

## 技術スタック

### コア
- **言語**: Rust (edition 2021)
- **非同期処理**: tokio
- **HTTP通信**: reqwest
- **HTMLパース**: scraper
- **シリアライゼーション**: serde, serde_json
- **CLI**: clap, rustyline
- **エラー処理**: anyhow, thiserror
- **ログ**: log, env_logger

### LLMインテグレーション
- **rig-core**: マルチプロバイダー対応のLLMフレームワーク
  - Ollama（ローカルモデル）
  - OpenAI API
  - Gemini API

### 環境設定
- **dotenvy**: `.env`ファイルからの環境変数読み込み

### 情報処理（予定）
- **PDF処理**: lopdf / pdf
- **全文検索**: tantivy
- **ベクトルDB**: qdrant-client

## プロジェクト構造

```
copal/
├── Cargo.toml
├── src/
│   ├── main.rs              # エントリーポイント
│   ├── lib.rs               # ライブラリクレートルート
│   ├── agent/               # エージェント構築・ツール定義
│   │   ├── mod.rs
│   │   ├── builder.rs       # プロバイダー別エージェント生成
│   │   ├── web_fetch.rs     # Webフェッチツール
│   │   └── web_search.rs    # Web検索ツール
│   ├── cli/                 # CLIインターフェース
│   │   ├── mod.rs
│   │   ├── repl.rs          # インタラクティブモード (REPL)
│   │   └── history.rs       # 会話履歴管理
│   ├── collectors/          # 情報ソース
│   │   ├── mod.rs
│   │   └── web.rs           # Webスクレイピング
│   └── llm/                 # LLMクライアント
│       ├── mod.rs
│       ├── client.rs        # クライアントインターフェース
│       └── rig_client.rs    # rig-core実装
└── docs/
    └── design.md            # アーキテクチャ設計
```

## 開発コマンド

```bash
# ビルドと実行（ワンショットモード）
cargo run -- "質問文"

# インタラクティブモード（REPL）
cargo run -- -i

# テスト
cargo test

# フォーマット
cargo fmt

# リント
cargo clippy

# リリースビルド
cargo build --release
```

## プロバイダー設定

環境変数または `.env` ファイルで設定:

| 変数 | 説明 | デフォルト |
|------|------|-----------|
| `LLM_PROVIDER` | プロバイダー名 (`ollama`, `openai`, `gemini`) | `ollama` |
| `LLM_MODEL` | モデル名 | プロバイダーごとのデフォルト |
| `OPENAI_API_KEY` | OpenAI APIキー | - |
| `GEMINI_API_KEY` | Gemini APIキー | - |

## ドキュメント
- アーキテクチャ設計: [docs/design.md](docs/design.md)

## 現在の開発フェーズ
フェーズ1: 基盤実装
- [x] プロジェクト初期化
- [x] 基本的なCLI構造
- [x] モジュール分割
- [x] シンプルなウェブスクレイピング
- [x] LLMインテグレーション（マルチプロバイダー: Ollama / OpenAI / Gemini）
- [x] インタラクティブモード（REPL）
- [x] 会話履歴管理

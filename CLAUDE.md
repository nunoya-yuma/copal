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
- **CLI**: rustyline (optional, behind `cli` feature flag)
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
│   │   ├── web_fetch.rs     # Webフェッチツール（Clone対応、キャッシュ共有）
│   │   ├── web_search.rs    # Web検索ツール
│   │   └── pdf_read.rs      # PDF読み取りツール
│   ├── cli/                 # CLIインターフェース（feature "cli" でゲート）
│   │   ├── mod.rs
│   │   ├── repl.rs          # インタラクティブモード (REPL)
│   │   └── render.rs        # ターミナルMarkdownレンダリング
│   ├── session/             # セッション管理（CLI/Web共通）
│   │   ├── mod.rs
│   │   └── history.rs       # 会話履歴管理
│   ├── collectors/          # 情報ソース
│   │   ├── mod.rs
│   │   ├── web.rs           # Webスクレイピング
│   │   ├── robots.rs        # robots.txtキャッシュ（Arc共有）
│   │   └── pdf.rs           # PDFテキスト抽出
│   ├── llm/                 # LLMクライアント
│   │   ├── mod.rs
│   │   ├── client.rs        # クライアントインターフェース
│   │   └── rig_client.rs    # rig-core実装
│   └── web/                 # Webバックエンド（feature "web" でゲート）
│       ├── mod.rs
│       ├── auth.rs          # Bearer token認証ミドルウェア
│       └── ...
├── frontend/                # React + TypeScript フロントエンド
│   ├── package.json
│   ├── vite.config.ts
│   └── src/
│       ├── App.tsx
│       ├── components/      # UIコンポーネント
│       ├── hooks/           # カスタムフック（useChat, useAuth）
│       └── utils/           # API通信など
└── docs/
    └── design.md            # アーキテクチャ設計
```

## 開発コマンド

```bash
# 実行（CLIモード、デフォルト）
cargo run

# 実行（Webサーバーモード、ポート3000）
cargo web   # = cargo run --no-default-features --features web

# テスト（全機能）
cargo test

# テスト（Webモード）
cargo test-web  # = cargo test --no-default-features --features web

# フォーマット
cargo fmt

# リント
cargo clippy

# リリースビルド
cargo build --release
```

### フロントエンド

```bash
cd frontend

npm ci               # 依存関係インストール
npm run dev          # 開発サーバー起動（localhost:5173）
npm run typecheck    # 型チェック
npm run test:ci      # テスト（1回実行）
npm run test         # テスト（ウォッチモード）
npm run build        # 本番ビルド → frontend/dist/
```

## プロバイダー設定

環境変数または `.env` ファイルで設定:

| 変数 | 説明 | デフォルト |
|------|------|-----------|
| `LLM_PROVIDER` | プロバイダー名 (`ollama`, `openai`, `gemini`) | `ollama` |
| `LLM_MODEL` | モデル名 | プロバイダーごとのデフォルト |
| `OPENAI_API_KEY` | OpenAI APIキー | - |
| `GEMINI_API_KEY` | Gemini APIキー | - |
| `COPAL_API_TOKEN` | Bearer認証トークン（Webモード必須） | - |
| `TAVILY_API_KEY` | Tavily Web検索APIキー | - |

## デプロイ情報

### デプロイ方法
```bash
# 自動デプロイ（GitHub Actions）
git push origin main
```

### 環境変数設定
- **ローカル開発** (`.env`): `LLM_PROVIDER=ollama`（デフォルト）
- **Azure本番**: `LLM_PROVIDER=openai` または `gemini`（環境変数で設定）

### 重要な注意事項
- **ターゲットポート**: 8080
- **APIキー**: Azure Container Apps のシークレットで管理
- **Ollama**: ローカル開発専用（Azure環境では使用不可）
- **環境固有の情報**（URL・リソース名）はローカルメモリで管理（public repoには記載しない）

## ドキュメント
- アーキテクチャ設計: [docs/design.md](docs/design.md)
- **デプロイガイド**: [docs/deployment.md](docs/deployment.md)（環境変数設定、履歴管理）
- デプロイ計画: `~/.claude/plans/distributed-sniffing-diffie.md`

## 現在の開発フェーズ
フェーズ1: 基盤実装
- [x] プロジェクト初期化
- [x] 基本的なCLI構造
- [x] モジュール分割
- [x] シンプルなウェブスクレイピング
- [x] LLMインテグレーション（マルチプロバイダー: Ollama / OpenAI / Gemini）
- [x] インタラクティブモード（REPL）
- [x] 会話履歴管理

フェーズ1.5: クラウド対応リファクタリング
- [x] ConversationHistoryをsessionモジュールに分離
- [x] RobotsCacheをArc<Mutex>でClone対応
- [x] WebFetchをAgent Builderに外部注入
- [x] Feature フラグでCLI/Web依存を分離

フェーズ2: Axum Webバックエンド
- [x] Axum WebサーバーとSSEストリーミング実装
- [x] React + TypeScript フロントエンド
- [x] セッション管理とマルチターン会話
- [x] Markdownレンダリング（react-markdown）
- [ ] Syntax highlighting（react-syntax-highlighter - TODO）

フェーズ3: Azure Container Apps デプロイ
- [x] Dockerマルチステージビルド
- [x] GitHub Actions CI/CD
- [x] Azure Container Registry
- [x] Azure Container Apps
- [x] 本番環境デプロイ成功
- [x] Bearer token認証（COPAL_API_TOKEN）
- [x] フロントエンドCI（typecheck / test / build）

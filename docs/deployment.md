# Deployment Guide

## Azure Container Apps 環境変数設定

### 設定方法の比較

| 方法 | メリット | デメリット | 用途 |
|------|---------|-----------|------|
| **手動CLI** | 即座に反映 | 履歴が残らない | 緊急時、初回セットアップ |
| **GitHub Actions** | コード管理、監査可能 | デプロイ必要 | 通常運用（推奨） |
| **Azure Portal** | 視覚的に確認 | コード化されない | 確認・検証 |

---

## 推奨フロー（GitHub Actions経由）

### 1. GitHub Secretsに保存

```bash
# GitHub CLIでシークレット設定（リポジトリ: copal）
gh secret set OPENAI_API_KEY --body "sk-proj-..."
gh secret set LLM_PROVIDER --body "openai"
gh secret set GEMINI_API_KEY --body "..."
gh secret set TAVILY_API_KEY --body "..."
```

または Web UI: https://github.com/{owner}/copal/settings/secrets/actions

### 2. デプロイ

```bash
# mainブランチにpushで自動デプロイ
git push origin main

# または手動トリガー
gh workflow run deploy.yml
```

---

## 緊急時の手動設定

```bash
# シークレット値を直接更新（即座に反映、リビジョン自動作成）
az containerapp secret set \
  --name copal-app \
  --resource-group copal-rg \
  --secrets \
    llm-provider="openai" \
    openai-api-key="sk-proj-..."
```

---

## 設定確認

### 環境変数一覧
```bash
az containerapp show \
  --name copal-app \
  --resource-group copal-rg \
  --query "properties.template.containers[0].env" \
  -o table
```

### シークレット一覧（値は表示されない）
```bash
az containerapp secret list \
  --name copal-app \
  --resource-group copal-rg \
  -o table
```

### リビジョン履歴（変更追跡）
```bash
az containerapp revision list \
  --name copal-app \
  --resource-group copal-rg \
  --query "[].{Name:name,Active:properties.active,Created:properties.createdTime}" \
  -o table
```

### 現在のアクティブリビジョンの詳細
```bash
az containerapp revision show \
  --name copal-app \
  --resource-group copal-rg \
  --revision $(az containerapp revision list --name copal-app --resource-group copal-rg --query "[?properties.active].name" -o tsv)
```

---

## 環境変数の構成

| 変数名 | 説明 | デフォルト | 必須 |
|--------|------|-----------|------|
| `LLM_PROVIDER` | LLMプロバイダー (`ollama`/`openai`/`gemini`) | `ollama` | ✓ |
| `OPENAI_API_KEY` | OpenAI APIキー | - | `LLM_PROVIDER=openai`の場合 |
| `GEMINI_API_KEY` | Gemini APIキー | - | `LLM_PROVIDER=gemini`の場合 |
| `TAVILY_API_KEY` | Tavily検索APIキー | - | Web検索機能利用時 |

---

## トラブルシューティング

### アプリが起動しない
1. リビジョンログを確認:
   ```bash
   az containerapp logs show \
     --name copal-app \
     --resource-group copal-rg \
     --follow
   ```
2. 環境変数が正しく設定されているか確認
3. APIキーが有効か確認（OpenAI Dashboardなど）

### シークレット更新が反映されない
- リビジョンが自動作成されるまで数分待つ
- `az containerapp revision list`でアクティブリビジョンを確認

### GitHub Actionsデプロイ失敗
1. Actionsタブでエラーログ確認
2. GitHub Secretsが設定されているか確認
3. `AZURE_CREDENTIALS`などの認証情報が有効か確認

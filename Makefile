.PHONY: help setup check fmt lint test build run migration-generate migration-run migration-revert clean install-hooks setup-test-db test-integration test-e2e

# デフォルトターゲット: ヘルプを表示
help:
	@echo "Available targets:"
	@echo "  setup              - 初期セットアップ (依存関係インストール + フック設定)"
	@echo "  check              - すべてのチェック実行 (fmt + lint + test)"
	@echo "  fmt                - コードフォーマット"
	@echo "  fmt-check          - フォーマットチェック（変更なし）"
	@echo "  lint               - Clippy実行"
	@echo "  test               - テスト実行"
	@echo "  test-coverage      - カバレッジ付きテスト実行"
	@echo "  test-integration   - 統合テスト実行（DB接続必要）"
	@echo "  test-e2e           - E2Eテスト実行（DB接続必要）"
	@echo "  setup-test-db      - テスト用データベースのセットアップ"
	@echo "  build              - リリースビルド"
	@echo "  build-dev          - デバッグビルド"
	@echo "  run                - アプリケーション実行"
	@echo "  migration-generate - マイグレーションファイル生成 (name=xxx)"
	@echo "  migration-run      - マイグレーション実行"
	@echo "  migration-revert   - マイグレーションロールバック"
	@echo "  clean              - ビルド成果物削除"
	@echo "  install-hooks      - Git hooksをインストール"
	@echo "  audit              - セキュリティ監査"

# 初期セットアップ
setup: install-hooks
	@echo "✅ Setting up development environment..."
	cargo build
	@echo "✅ Setup complete!"

# すべてのチェック実行
check: fmt-check lint test
	@echo "✅ All checks passed!"

# コードフォーマット
fmt:
	@echo "🔧 Formatting code..."
	cargo fmt

# フォーマットチェック
fmt-check:
	@echo "🔍 Checking code format..."
	cargo fmt --check

# Clippy実行
lint:
	@echo "🔍 Running clippy..."
	cargo clippy -- -D warnings

# テスト実行
test:
	@echo "🧪 Running tests..."
	cargo test

# 統合テスト実行（データベース接続必要）
test-integration:
	@echo "🧪 Running integration tests (database required)..."
	@echo "⚠️  TEST_DATABASE_URL環境変数が設定されていることを確認してください"
	cargo test -- --ignored

# E2Eテスト実行（データベース接続必要）
test-e2e:
	@echo "🧪 Running E2E tests (database required)..."
	@echo "⚠️  TEST_DATABASE_URL環境変数が設定されていることを確認してください"
	cargo test --test e2e_tests -- --ignored --test-threads=1

# カバレッジ付きテスト
test-coverage:
	@echo "🧪 Running tests with coverage..."
	cargo tarpaulin --out Html --output-dir coverage

# テスト用データベースのセットアップ
setup-test-db:
	@echo "🗄️ Setting up test database..."
	@./scripts/setup-test-db.sh

# リリースビルド
build:
	@echo "🔨 Building release binary..."
	cargo build --release

# デバッグビルド
build-dev:
	@echo "🔨 Building debug binary..."
	cargo build

# アプリケーション実行
run:
	@echo "🚀 Running application..."
	cargo run

# マイグレーションファイル生成
migration-generate:
ifndef name
	@echo "❌ Error: name parameter is required"
	@echo "Usage: make migration-generate name=create_table"
	@exit 1
endif
	@echo "📝 Generating migration: $(name)"
	diesel migration generate $(name)

# マイグレーション実行
migration-run:
	@echo "🔄 Running migrations..."
	diesel migration run

# マイグレーションロールバック
migration-revert:
	@echo "⏪ Reverting last migration..."
	diesel migration revert

# ビルド成果物削除
clean:
	@echo "🧹 Cleaning build artifacts..."
	cargo clean

# Git hooksをインストール
install-hooks:
	@echo "🔗 Installing git hooks..."
	@if [ -f .githooks/pre-commit ]; then \
		chmod +x .githooks/pre-commit; \
		git config core.hooksPath .githooks; \
		echo "✅ Git hooks installed successfully!"; \
	else \
		echo "❌ Error: .githooks/pre-commit not found"; \
		exit 1; \
	fi

# セキュリティ監査
audit:
	@echo "🔒 Running security audit..."
	cargo audit

# Docker関連コマンド
docker-build:
	@echo "🐳 Building Docker image..."
	docker build -t onsen_tabi:latest .

docker-run:
	@echo "🐳 Running Docker container..."
	docker run -p 8000:8000 --env-file .env onsen_tabi:latest

# 開発環境用: ウォッチモード
watch:
	@echo "👀 Running in watch mode..."
	cargo watch -x run

# データベースセットアップ
db-setup:
	@echo "🗄️ Setting up database..."
	diesel setup
	@echo "✅ Database setup complete!"

# データベースリセット
db-reset:
	@echo "⚠️  Resetting database..."
	diesel database reset
	@echo "✅ Database reset complete!"

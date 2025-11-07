# onsen_tabi
[![codecov](https://codecov.io/gh/konabe/onsen_tabi/graph/badge.svg?token=WRRRJTB2BE)](https://codecov.io/gh/konabe/onsen_tabi)
[![CI](https://github.com/konabe/onsen_tabi/workflows/CI/badge.svg)](https://github.com/konabe/onsen_tabi/actions)

## what's this

温泉を起点とした旅の記録をしてみたいと思ったので作りました。

## 目次

- [システム構成](#システム構成)
- [開発環境のセットアップ](#開発環境のセットアップ)
- [開発ワークフロー](#開発ワークフロー)
- [マイグレーション](#マイグレーション)
- [Docker](#docker)
- [ガードレール](#ガードレール)
- [テストカバレッジ](#テストカバレッジ)

## システム構成

vercel <-> Route53 <-> ALB <-> EC2 <-> RDS for MySQL

- [frontend source](https://github.com/konabe/onsen-tabi-web)

## 開発環境のセットアップ

### 必要な環境

- Rust (最新安定版)
- Diesel CLI
- MySQL 8.0+
- Docker (オプション)

### 初期セットアップ

1. **リポジトリのクローン**
   ```bash
   git clone https://github.com/konabe/onsen_tabi.git
   cd onsen_tabi
   ```

2. **依存関係のインストール**
   ```bash
   # Diesel CLIのインストール（MySQLサポート付き）
   cargo install diesel_cli --no-default-features --features mysql
   
   # cargo-watchのインストール（開発時の自動リロード用）
   cargo install cargo-watch
   
   # cargo-tarpaulinのインストール（カバレッジ計測用）
   cargo install cargo-tarpaulin
   ```

3. **環境変数の設定**
   ```bash
   cp .env.sample .env
   # .envファイルを編集してデータベース接続情報とJWTシークレットを設定
   ```

4. **データベースのセットアップ**
   ```bash
   # データベースとテーブルの作成
   make db-setup
   # または
   diesel setup
   ```

5. **Git hooksのインストール**
   ```bash
   make install-hooks
   ```

6. **ビルドとテスト**
   ```bash
   make check
   ```

## 開発ワークフロー

### よく使うコマンド

開発を効率化するため、Makefileを用意しています。

```bash
# ヘルプを表示
make help

# 開発前のチェック（フォーマット + Lint + テスト）
make check

# コードフォーマット
make fmt

# Lintチェック（Clippy）
make lint

# テスト実行
make test

# アプリケーション実行
make run

# ウォッチモード（コード変更時に自動リロード）
make watch
```

### ブランチ戦略

```bash
# 新機能の開発
git checkout -b feature/<issue-number>-<description>

# バグ修正
git checkout -b fix/<issue-number>-<description>

# リファクタリング
git checkout -b refactor/<description>

# ドキュメント更新
git checkout -b docs/<description>
```

### コミット規約

Conventional Commitsに従ってコミットメッセージを記述してください。

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Type一覧**:
- `feat`: 新機能
- `fix`: バグ修正
- `refactor`: リファクタリング
- `docs`: ドキュメント変更
- `test`: テスト追加・修正
- `chore`: ビルド、設定変更

**例**:
```bash
git commit -m "feat(onsen): 泉質自動生成機能を追加"
git commit -m "fix(auth): JWT有効期限のバグを修正"
git commit -m "docs: READMEにセットアップ手順を追加"
```

## マイグレーション

### マイグレーションファイルの作成

```bash
# Makefileを使用（推奨）
make migration-generate name=create_new_table

# または直接Dieselコマンドを実行
diesel migration generate create_new_table
```

### マイグレーションの実行

```bash
# マイグレーション実行
make migration-run

# または
diesel migration run
```

### マイグレーションのロールバック

```bash
# 最後のマイグレーションを戻す
make migration-revert

# または
diesel migration revert
```

### マイグレーションのベストプラクティス

1. **up.sqlとdown.sqlは必ずペア**で作成
2. **down.sqlで完全にロールバック**できることを確認
3. **外部キー制約**を適切に設定
4. **インデックス**を必要に応じて追加
5. **コメント**でカラムの意図を明記

## Docker

### Dockerイメージのビルド

```bash
# Makefileを使用
make docker-build

# または直接ビルド
docker build --no-cache --tag konabe/onsen_tabi:latest .
```

### Dockerコンテナの実行

```bash
# Makefileを使用（.envファイルが必要）
make docker-run

# または環境変数を指定して実行
docker run --rm \
  --env DATABASE_URL="mysql://user:pass@host:3306/db" \
  --env JWT_SECRET_KEY="your-secret-key" \
  --publish 8000:8000 \
  --name web_server \
  konabe/onsen_tabi:latest
```

## ガードレール

このプロジェクトはAI駆動開発に対応したガードレールを実装しています。

### Git Pre-commitフック

コミット前に以下のチェックが自動実行されます：

- **単体テスト**: `cargo test`（データベース不要）
- **統合テスト**: `cargo test -- --ignored`（データベース接続時のみ）

```bash
# pre-commitフックの確認
cat .git/hooks/pre-commit

# pre-commitフックのテスト実行
.git/hooks/pre-commit
```

**注意**: 
- テストデータベースが起動している場合は統合テストも実行されます
- サーバーE2Eテスト（`e2e_server_tests`）は手動実行が必要です

### 手動チェック（追加のコードチェック）

```bash
# フォーマットチェック
cargo fmt --check

# Lintチェック
cargo clippy -- -D warnings

# すべてのチェックを実行
make check

# セキュリティ監査
make audit
```

### 旧自動チェック（参考）

以前のガードレール設定では以下が自動実行されていました：

- **フォーマットチェック**: `cargo fmt --check`
- **Lintチェック**: `cargo clippy -- -D warnings`
- **テスト**: `cargo test`

### 手動チェック

```bash
# すべてのチェックを実行
make check

# セキュリティ監査
make audit
```

### ガードレールドキュメント

詳細なガードレールの説明は以下のドキュメントを参照してください：

- **[doc/AI_GUARDRAILS.md](doc/AI_GUARDRAILS.md)**: AI駆動開発のためのガードレール
- **[doc/DESIGN.md](doc/DESIGN.md)**: システム設計書

### 主なガードレール項目

1. **アーキテクチャ**: Clean Architectureのレイヤー分離
2. **データベース**: マイグレーション管理、NULL許容の明示
3. **API設計**: RESTful原則、認証・認可
4. **セキュリティ**: パスワードハッシュ化、JWT管理、SQLインジェクション対策
5. **コード品質**: 命名規則、エラーハンドリング、テストカバレッジ
6. **Git管理**: コミットメッセージ、ブランチ戦略

## テスト

### テストの種類

このプロジェクトには3種類のテストがあります：

1. **単体テスト**: データベース接続不要（126テスト）
2. **アプリケーション統合テスト**: データベース接続必要（27テスト）
   - `controller_tests.rs`: コントローラ層のテスト（10テスト）
   - `e2e_app_tests.rs`: Rocket内部クライアントを使用したE2Eテスト（5テスト）
   - `repository_tests.rs`: リポジトリ層のテスト（12テスト）
3. **サーバーE2Eテスト**: 実際のHTTPサーバー接続が必要（8テスト）
   - `e2e_server_tests.rs`: reqwestを使用した実際のHTTPリクエストテスト

### クイックスタート

```bash
# 1. テスト用データベースのセットアップ
make setup-test-db

# 2. 単体テストのみ実行（DB不要）
cargo test

# 3. 統合テスト実行（データベース接続必要）
cargo test -- --ignored

# 4. サーバーE2Eテスト実行（サーバー起動が必要）
# ターミナル1: サーバー起動
cargo run

# ターミナル2: E2Eテスト実行
cargo test --test e2e_server_tests -- --ignored
```

### テスト用データベースのセットアップ（詳細）

#### 自動セットアップ（推奨）

```bash
# .envファイルにTEST_DATABASE_URLを設定後
make setup-test-db
```

このコマンドは以下を自動実行します：
- テストDBの作成（存在しない場合）
- マイグレーションの実行
- テーブルの確認

#### 手動セットアップ

```bash
# 1. テスト用データベースの作成
mysql -u root -p
CREATE DATABASE onsen_tabi_test CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
GRANT ALL PRIVILEGES ON onsen_tabi_test.* TO 'root'@'localhost';
FLUSH PRIVILEGES;

# 2. .envに追加
echo "TEST_DATABASE_URL=mysql://root:password@localhost:3306/onsen_tabi_test" >> .env

# 3. マイグレーション実行
DATABASE_URL=$TEST_DATABASE_URL diesel migration run
```

### テスト実行コマンド

```bash
# 単体テストのみ（デフォルト、DB不要）
cargo test

# 統合テスト（データベース接続必要）
cargo test -- --ignored

# 特定のテストファイルのみ実行
cargo test --test controller_tests -- --ignored
cargo test --test e2e_app_tests -- --ignored
cargo test --test repository_tests -- --ignored

# サーバーE2Eテスト（サーバー起動が必要）
cargo test --test e2e_server_tests -- --ignored

# カバレッジ付きテスト
make test-coverage
```

### Git Pre-commitフック

コミット時に自動的にテストが実行されます：

```bash
# コミット時の自動実行内容
# 1. 単体テスト（常に実行）
# 2. 統合テスト（テストDBが起動している場合のみ）
# 3. サーバーE2Eテストは手動実行が必要

# pre-commitフックの内容確認
cat .git/hooks/pre-commit

# pre-commitフックの手動テスト
.git/hooks/pre-commit
```

### テスト詳細

詳細なテスト実行方法は以下を参照：
- **[tests/README.md](tests/README.md)**: 統合テスト実行ガイド
- **[doc/TEST_COVERAGE.md](doc/TEST_COVERAGE.md)**: テストカバレッジレポート

### テスト統計

- **総テスト数**: 161
  - 単体テスト: 126
  - 統合テスト: 27（controller: 10, e2e_app: 5, repository: 12）
  - サーバーE2Eテスト: 8

## テストカバレッジ

![sunburst](https://codecov.io/gh/konabe/onsen_tabi/graphs/sunburst.svg?token=WRRRJTB2BE)
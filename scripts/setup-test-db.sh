#!/bin/bash
# =============================================================================
# テスト用データベースセットアップスクリプト
# =============================================================================
#
# このスクリプトは、統合テストとE2Eテスト用のデータベースを自動的にセットアップします。
#
# 使用方法:
#   ./scripts/setup-test-db.sh
#
# または
#   make setup-test-db

set -e  # エラーが発生したら即座に終了

# 色付きの出力用
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}テスト用データベースのセットアップ${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""

# 環境変数の読み込み
if [ -f .env ]; then
    echo -e "${GREEN}✓${NC} .env ファイルを読み込んでいます..."
    export $(cat .env | grep -v '^#' | xargs)
else
    echo -e "${YELLOW}⚠${NC} .env ファイルが見つかりません"
    echo -e "${YELLOW}→${NC} .env.sample をコピーして .env を作成してください"
    exit 1
fi

# DATABASE_URLからデータベース情報を抽出
if [ -z "$DATABASE_URL" ]; then
    echo -e "${RED}✗${NC} DATABASE_URL が設定されていません"
    exit 1
fi

# TEST_DATABASE_URLの確認
if [ -z "$TEST_DATABASE_URL" ]; then
    echo -e "${YELLOW}⚠${NC} TEST_DATABASE_URL が設定されていません"
    echo -e "${YELLOW}→${NC} .env に TEST_DATABASE_URL を追加してください"
    exit 1
fi

# データベース接続情報の抽出（簡易版）
DB_USER=$(echo $DATABASE_URL | sed -n 's/.*:\/\/\([^:]*\):.*/\1/p')
DB_PASS=$(echo $DATABASE_URL | sed -n 's/.*:\/\/[^:]*:\([^@]*\)@.*/\1/p')
DB_HOST=$(echo $DATABASE_URL | sed -n 's/.*@\([^:]*\):.*/\1/p')
DB_PORT=$(echo $DATABASE_URL | sed -n 's/.*:\([0-9]*\)\/.*/\1/p')

TEST_DB_NAME=$(echo $TEST_DATABASE_URL | sed -n 's/.*\/\(.*\)/\1/p')

echo -e "${GREEN}✓${NC} データベース接続情報:"
echo -e "  ユーザー: ${DB_USER}"
echo -e "  ホスト: ${DB_HOST}"
echo -e "  ポート: ${DB_PORT}"
echo -e "  テストDB名: ${TEST_DB_NAME}"
echo ""

# MySQLが利用可能か確認
echo -e "${GREEN}→${NC} MySQLサーバーへの接続を確認中..."
if ! mysql -h${DB_HOST} -P${DB_PORT} -u${DB_USER} -p${DB_PASS} -e "SELECT 1;" > /dev/null 2>&1; then
    echo -e "${RED}✗${NC} MySQLサーバーに接続できません"
    echo -e "${RED}→${NC} MySQLが起動しているか、接続情報が正しいか確認してください"
    exit 1
fi
echo -e "${GREEN}✓${NC} MySQLサーバーに接続しました"
echo ""

# テスト用データベースの存在確認
echo -e "${GREEN}→${NC} テスト用データベースの確認中..."
DB_EXISTS=$(mysql -h${DB_HOST} -P${DB_PORT} -u${DB_USER} -p${DB_PASS} -e "SHOW DATABASES LIKE '${TEST_DB_NAME}';" | grep ${TEST_DB_NAME} || echo "")

if [ -z "$DB_EXISTS" ]; then
    echo -e "${YELLOW}→${NC} テスト用データベースが存在しません。作成します..."
    mysql -h${DB_HOST} -P${DB_PORT} -u${DB_USER} -p${DB_PASS} <<EOF
CREATE DATABASE ${TEST_DB_NAME} CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
EOF
    echo -e "${GREEN}✓${NC} テスト用データベース '${TEST_DB_NAME}' を作成しました"
else
    echo -e "${GREEN}✓${NC} テスト用データベース '${TEST_DB_NAME}' は既に存在します"
fi
echo ""

# マイグレーションの実行
echo -e "${GREEN}→${NC} マイグレーションを実行中..."
if ! command -v diesel &> /dev/null; then
    echo -e "${YELLOW}⚠${NC} diesel CLI がインストールされていません"
    echo -e "${YELLOW}→${NC} インストールしています..."
    cargo install diesel_cli --no-default-features --features mysql
fi

DATABASE_URL=${TEST_DATABASE_URL} diesel migration run
echo -e "${GREEN}✓${NC} マイグレーションが完了しました"
echo ""

# 確認
echo -e "${GREEN}→${NC} テーブルの確認..."
TABLE_COUNT=$(mysql -h${DB_HOST} -P${DB_PORT} -u${DB_USER} -p${DB_PASS} -D${TEST_DB_NAME} -e "SHOW TABLES;" | wc -l)
echo -e "${GREEN}✓${NC} ${TABLE_COUNT} 個のテーブルが作成されました"
echo ""

echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}セットアップ完了！${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""
echo -e "以下のコマンドでテストを実行できます:"
echo ""
echo -e "  ${YELLOW}# 統合テスト実行${NC}"
echo -e "  cargo test -- --ignored"
echo ""
echo -e "  ${YELLOW}# E2Eテスト実行${NC}"
echo -e "  cargo test --test e2e_tests -- --ignored"
echo ""
echo -e "  ${YELLOW}# 直列実行（競合回避）${NC}"
echo -e "  cargo test -- --ignored --test-threads=1"
echo ""

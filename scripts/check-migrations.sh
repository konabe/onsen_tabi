#!/bin/bash
# マイグレーションファイルの整合性チェック
# CI および pre-commit フックから使用されます

set -e

echo "Checking migration files structure..."

# 新しく追加されたマイグレーションディレクトリを取得
if git rev-parse --verify origin/main >/dev/null 2>&1; then
  CHANGED_MIGRATIONS=$(git diff --name-only origin/main...HEAD | grep "^migrations/" | sed 's|/[^/]*$||' | sort -u)
else
  # origin/main が存在しない場合（初回クローン等）は全マイグレーションをチェック
  CHANGED_MIGRATIONS=$(find migrations -mindepth 1 -maxdepth 1 -type d | sort)
fi

if [ -z "$CHANGED_MIGRATIONS" ]; then
  echo "No new migrations detected"
  exit 0
fi

echo "New migrations detected:"
echo "$CHANGED_MIGRATIONS"

# 各マイグレーションディレクトリをチェック
EXIT_CODE=0
for migration_dir in $CHANGED_MIGRATIONS; do
  echo "Checking $migration_dir..."
  
  if [ ! -f "$migration_dir/up.sql" ]; then
    echo "❌ Missing up.sql in $migration_dir"
    EXIT_CODE=1
  fi
  
  if [ ! -f "$migration_dir/down.sql" ]; then
    echo "❌ Missing down.sql in $migration_dir"
    EXIT_CODE=1
  fi
  
  if [ -f "$migration_dir/up.sql" ] && [ -f "$migration_dir/down.sql" ]; then
    echo "✅ Migration files are valid for $migration_dir"
  fi
done

exit $EXIT_CODE

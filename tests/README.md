# 統合テスト実行ガイド

このディレクトリには、onsen_tabiプロジェクトの統合テストとE2Eテストが含まれています。

## テストファイル

### 1. `controller_tests.rs`
APIエンドポイントの統合テスト（Rocketテストクライアント使用）

**テスト内容**:
- エンドポイントのHTTPリクエスト/レスポンス
- 認証・認可フロー
- エラーハンドリング
- ステータスコードの検証

**実装状況**: ✅ 完了（10テスト）

### 2. `repository_tests.rs`
データベースアクセス層の統合テスト

**テスト内容**:
- CRUD操作
- トランザクション処理
- JOIN操作
- エラーハンドリング

**実装状況**: ✅ 完了（12テスト）

### 3. `e2e_tests.rs`
エンドツーエンドテスト

**テスト内容**:
- ユーザー登録→ログインフロー
- エリア→ホテル→温泉作成フロー
- 検索・取得フロー
- エラーハンドリングシナリオ

**実装状況**: ✅ 完了（5テスト）

---

## クイックスタート

### 自動セットアップ（推奨）

```bash
# 1. .env.sampleをコピーして.envを作成
cp .env.sample .env

# 2. .envを編集してデータベース接続情報を設定
# TEST_DATABASE_URL=mysql://root:password@localhost:3306/onsen_tabi_test

# 3. テスト用データベースを自動セットアップ
make setup-test-db

# 4. テスト実行
make test-integration  # 統合テスト
make test-e2e          # E2Eテスト
```

---

## セットアップ（手動）

### 前提条件

1. **MySQL/MariaDB**: テスト用データベースが必要
2. **Diesel CLI**: マイグレーション実行用
3. **環境変数**: テスト用データベース接続情報

### テスト用データベースの準備

#### 1. データベース作成

```sql
CREATE DATABASE onsen_tabi_test CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
GRANT ALL PRIVILEGES ON onsen_tabi_test.* TO 'root'@'localhost';
FLUSH PRIVILEGES;
```

#### 2. 環境変数設定

`.env`ファイルに追加:

```bash
TEST_DATABASE_URL=mysql://root:password@localhost:3306/onsen_tabi_test
JWT_SECRET_KEY=test_secret_key_for_integration_testing_only
```

または直接環境変数をエクスポート:

```bash
export TEST_DATABASE_URL="mysql://root:password@localhost:3306/onsen_tabi_test"
export JWT_SECRET_KEY="test_secret_key_for_integration_testing_only"
```

#### 3. Diesel CLIのインストール

```bash
cargo install diesel_cli --no-default-features --features mysql
```

#### 4. マイグレーション実行

```bash
DATABASE_URL=$TEST_DATABASE_URL diesel migration run
```

---

## テスト実行

### すべてのテストを実行

```bash
# 単体テストのみ（統合テストは除く）
cargo test

# 統合テストとE2Eテストを実行（データベース接続必要）
cargo test -- --ignored --test-threads=1

# Makefileを使用（推奨）
make test              # 単体テストのみ
make test-integration  # 統合テスト
make test-e2e          # E2Eテスト
```

### 特定の統合テストを実行

```bash
# Controller統合テスト
cargo test --test controller_tests -- --ignored

# Repository統合テスト
cargo test --test repository_tests -- --ignored

# E2Eテスト
cargo test --test e2e_tests -- --ignored --test-threads=1

# 特定のテスト関数
cargo test test_database_connection --test repository_tests -- --ignored
```

### 詳細出力で実行

```bash
# 標準出力を表示
cargo test -- --ignored --nocapture

# テストの進行状況を表示
cargo test -- --ignored --nocapture --test-threads=1
```

---

## テスト実装の進め方

### Controller統合テストの実装

1. **Rocketインスタンスのセットアップ**
   ```rust
   fn rocket() -> rocket::Rocket<rocket::Build> {
       rocket::build()
           .mount("/", routes![/* ... */])
   }
   ```

2. **テストクライアントの作成**
   ```rust
   let client = Client::tracked(rocket()).expect("valid rocket instance");
   ```

3. **テストの実装**
   ```rust
   #[test]
   fn test_endpoint() {
       let client = Client::tracked(rocket()).unwrap();
       let response = client.get("/area").dispatch();
       assert_eq!(response.status(), Status::Ok);
   }
   ```

### Repository統合テストの実装

1. **テストデータベース接続**
   ```rust
   let mut conn = establish_test_connection().unwrap();
   ```

2. **テストデータの準備**
   ```rust
   // トランザクション内でテスト
   conn.test_transaction::<_, Error, _>(|conn| {
       // テストコード
       Ok(())
   })
   ```

3. **アサーション**
   ```rust
   let result = area_repository::get_area(1);
   assert!(result.is_some());
   ```

---

## CI/CDでの統合テスト

### GitHub Actions設定例

```yaml
- name: Setup test database
  run: |
    mysql -e "CREATE DATABASE onsen_tabi_test;"
    diesel migration run --database-url=$TEST_DATABASE_URL
  env:
    TEST_DATABASE_URL: mysql://root:@127.0.0.1:3306/onsen_tabi_test

- name: Run integration tests
  run: cargo test -- --ignored --test-threads=1
  env:
    TEST_DATABASE_URL: mysql://root:@127.0.0.1:3306/onsen_tabi_test
    JWT_SECRET_KEY: test_secret_key
```

---

## トラブルシューティング

### データベース接続エラー

```
Error: Failed to connect to database
```

**解決方法**:
1. MySQLが起動しているか確認
2. `TEST_DATABASE_URL`が正しく設定されているか確認
3. データベースユーザーの権限を確認

### マイグレーションエラー

```
Error: Migration failed
```

**解決方法**:
1. Diesel CLIがインストールされているか確認
2. マイグレーションファイルが存在するか確認
3. データベースURLが正しいか確認

### テストがスキップされる

```
test result: ok. 0 passed; 0 failed; 5 ignored
```

**原因**: `#[ignore]`属性がついているテスト

**解決方法**:
```bash
cargo test -- --ignored
```

---

## ベストプラクティス

1. **テスト用データベースを使用**
   - 本番データベースを使用しない
   - テスト用の独立したデータベースを作成

2. **トランザクションを使用**
   - 各テストをトランザクション内で実行
   - テスト後に自動的にロールバック

3. **テストの独立性**
   - テスト間でデータを共有しない
   - 各テストは独立して実行可能にする

4. **クリーンアップ**
   - テスト後は必ずデータをクリーンアップ
   - `#[ignore]`を適切に使用

5. **並列実行の制御**
   - データベーステストは`--test-threads=1`で実行
   - データ競合を避ける

---

## 参考リンク

- [Rocket Testing Guide](https://rocket.rs/v0.5/guide/testing/)
- [Diesel Testing](https://diesel.rs/guides/getting-started.html)
- [Rust Test Organization](https://doc.rust-lang.org/book/ch11-03-test-organization.html)

---

最終更新: 2025-11-07

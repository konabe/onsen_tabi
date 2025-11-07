# AI駆動開発ガードレール

## 1. 概要

このドキュメントは、AI支援によるコード生成・変更において、品質・セキュリティ・一貫性を保つためのガードレールを定義します。

---

## 2. アーキテクチャガードレール

### 2.1 レイヤー分離の厳守

**ルール**:
- Application層はDomain層とInfrastructure層のみに依存可
- Domain層はInfrastructure層に依存してはならない
- Infrastructure層は他の層に依存してはならない

**チェック項目**:
```rust
// ❌ NG: Domain層がInfrastructure層に依存
// src/domain/onsen_entity.rs
use crate::infrastructure::mysql::diesel_model::DieselOnsen;

// ✅ OK: Domain層は自己完結
// src/domain/onsen_entity.rs
pub struct OnsenEntity {
    pub id: u32,
    // ...
}
```

**自動チェック**:
```bash
# 依存関係の逆転を検出
grep -r "use crate::infrastructure" src/domain/
```

### 2.2 ビルダーパターンの使用

**ルール**:
- 3つ以上のフィールドを持つエンティティは必ずビルダーパターンを実装
- 必須フィールドはビルド時にバリデーション

**テンプレート**:
```rust
pub struct EntityBuilder {
    field1: Option<Type1>,
    field2: Option<Type2>,
    // ...
}

impl EntityBuilder {
    pub fn new() -> Self {
        Self {
            field1: None,
            field2: None,
        }
    }
    
    pub fn field1(mut self, value: Type1) -> Self {
        self.field1 = Some(value);
        self
    }
    
    pub fn build(self) -> Entity {
        Entity {
            field1: self.field1.expect("field1 is required"),
            field2: self.field2.expect("field2 is required"),
        }
    }
}
```

---

## 3. データベースガードレール

### 3.1 マイグレーションの管理

**ルール**:
1. すべてのスキーマ変更は必ずマイグレーションを通して実行
2. マイグレーションファイルは手動編集禁止（diesel migration generateを使用）
3. up.sqlとdown.sqlは必ずペアで作成

**命名規則**:
```
YYYY-MM-DD-HHMMSS_descriptive_name/
  ├── up.sql    # スキーマ変更
  └── down.sql  # ロールバック処理
```

**チェックリスト**:
- [ ] マイグレーションファイル名が日時形式である
- [ ] up.sqlとdown.sqlが両方存在する
- [ ] down.sqlでup.sqlの変更を完全に戻せる
- [ ] 外部キー制約が適切に設定されている

### 3.2 NULL許容の明示

**ルール**:
- デフォルトは NOT NULL
- NULLを許容する場合は明示的な理由をコメントに記載

**例**:
```sql
CREATE TABLE hotel (
    id INT UNSIGNED NOT NULL PRIMARY KEY AUTO_INCREMENT,
    name VARCHAR(255) NOT NULL,
    -- 外部キーだが、どのエリアにも属さない宿泊所を許容するためNULL可
    area_id INT UNSIGNED NULL,
    FOREIGN KEY (area_id) REFERENCES area(id)
);
```

### 3.3 インデックス戦略

**ルール**:
- 外部キーには必ずインデックスを作成
- 検索条件に使用するカラムにインデックスを検討

**例**:
```sql
CREATE INDEX idx_hotel_area_id ON hotel(area_id);
CREATE INDEX idx_onsen_hotel_id ON onsen(hotel_id);
CREATE INDEX idx_onsen_area_id ON onsen(area_id);
```

---

## 4. APIガードレール

### 4.1 エンドポイント設計

**ルール**:
- RESTful原則に従う
- リソース指向のURL設計
- HTTPメソッドの適切な使用

**命名規則**:
```
GET    /resource          # 一覧取得
GET    /resource/<id>     # 詳細取得
POST   /resource          # 新規作成
PUT    /resource/<id>     # 更新
DELETE /resource/<id>     # 削除（未実装）
```

**禁止事項**:
```
❌ GET /getResource
❌ POST /createResource
❌ /resource/update/<id>
```

### 4.2 リクエスト/レスポンスモデル

**ルール**:
1. エンティティを直接公開しない
2. 必ず専用のRequest/Responseモデルを作成
3. camelCase命名（フロントエンドとの互換性）

**実装例**:
```rust
// ✅ OK: 専用モデル
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnsenRequest {
    pub name: String,
    pub area_id: Option<u32>,
    // ...
}

// ❌ NG: エンティティを直接使用
#[post("/onsen", data = "<entity>")]
pub fn post_onsen(entity: Json<OnsenEntity>) { }
```

### 4.3 認証・認可

**ルール**:
- 読み取り専用エンドポイント: 認証不要
- 作成・更新・削除エンドポイント: JWT認証必須

**実装チェックリスト**:
```rust
// ✅ 認証必須のエンドポイント
#[post("/resource", data = "<request>")]
pub fn create_resource(
    _user: ValidatedUser,  // ガード必須
    request: Json<Request>
) -> Json<Response>

// ✅ 認証不要のエンドポイント
#[get("/resource")]
pub fn get_resources() -> Json<Vec<Response>>
```

### 4.4 エラーハンドリング

**ルール**:
- データベースエラーは500 Internal Server Error
- 認証エラーは401 Unauthorized
- 存在しないリソースは404 Not Found
- バリデーションエラーは400 Bad Request

**実装例**:
```rust
pub fn get_resource(id: u32) -> Result<Json<Response>, Status> {
    match repository::get_resource(id) {
        Some(entity) => Ok(Json(entity.into())),
        None => Err(Status::NotFound)
    }
}
```

---

## 5. セキュリティガードレール

### 5.1 パスワード管理

**ルール**:
1. パスワードは必ずArgon2でハッシュ化
2. プレーンテキストのパスワードをログに出力しない
3. ソルトは自動生成（手動管理禁止）

**実装チェック**:
```rust
// ✅ OK
let hashed = crypto::create_hash(&plain_password);
user.hashed_password = hashed;

// ❌ NG
user.password = plain_password;
println!("Password: {}", plain_password);  // ログ出力禁止
```

### 5.2 JWT管理

**ルール**:
- シークレットキーは環境変数から読み込み
- ハードコーディング厳禁
- 有効期限は24時間以内

**チェック項目**:
```rust
// ✅ OK
let secret = env::var("JWT_SECRET_KEY")
    .expect("JWT_SECRET_KEY must be set");

// ❌ NG
let secret = "my-secret-key";  // ハードコーディング禁止
```

### 5.3 SQLインジェクション対策

**ルール**:
- 必ずDieselのクエリビルダーを使用
- 生のSQL文字列連結は禁止

**実装例**:
```rust
// ✅ OK: Dieselのクエリビルダー
use crate::schema::onsen::dsl::*;
onsen.filter(id.eq(target_id))
    .first::<Onsen>(&mut conn)

// ❌ NG: 文字列連結
let query = format!("SELECT * FROM onsen WHERE id = {}", target_id);
```

### 5.4 CORS設定

**ルール**:
- 本番環境では特定のオリジンのみ許可
- 開発環境のみワイルドカード許可

**実装チェック**:
```rust
// 開発環境
response.set_header(Header::new("Access-Control-Allow-Origin", "*"));

// 本番環境（要実装）
let allowed_origins = env::var("ALLOWED_ORIGINS")
    .unwrap_or_else(|_| "https://example.com".to_string());
response.set_header(Header::new("Access-Control-Allow-Origin", allowed_origins));
```

---

## 6. コード品質ガードレール

### 6.1 命名規則

**Rust標準に従う**:
```rust
// 型名: PascalCase
struct OnsenEntity { }
enum SpringLiquid { }

// 関数名・変数名: snake_case
fn get_onsen_by_id(id: u32) -> Option<OnsenEntity> { }
let area_id = 1;

// 定数: SCREAMING_SNAKE_CASE
const MAX_RETRIES: u32 = 3;

// ライフタイム・型パラメータ: 小文字1文字または短い名前
fn process<'a, T>(data: &'a T) { }
```

**禁止事項**:
```rust
// ❌ NG
struct onsenEntity { }      // PascalCaseでない
fn GetOnsen() { }           // camelCaseでない
let AreaId = 1;             // snake_caseでない
```

### 6.2 コメント規則

**ルール**:
1. 公開API（pub）には必ずドキュメントコメント
2. 複雑なビジネスロジックには説明コメント
3. TODOコメントには担当者と期限を記載

**実装例**:
```rust
/// 指定されたIDの温泉情報を取得する
///
/// # Arguments
/// * `id` - 温泉ID
///
/// # Returns
/// * `Some(OnsenEntity)` - 温泉が見つかった場合
/// * `None` - 温泉が見つからない場合
pub fn get_onsen(id: u32) -> Option<OnsenEntity> {
    // TODO(konabe): キャッシュ機能を追加 (2025-12-31まで)
    repository::get_onsen(id)
}
```

### 6.3 エラーハンドリング

**ルール**:
- `unwrap()` の使用は禁止（テストコードを除く）
- `expect()` には必ず理由を記載
- `Result` と `Option` を適切に使い分け

**実装例**:
```rust
// ✅ OK
let secret = env::var("JWT_SECRET_KEY")
    .expect("JWT_SECRET_KEY must be set in environment");

let onsen = match get_onsen(id) {
    Some(o) => o,
    None => return Err(Status::NotFound)
};

// ❌ NG
let secret = env::var("JWT_SECRET_KEY").unwrap();  // パニックの理由不明
```

### 6.4 テストカバレッジ

**ルール**:
- ドメインロジックは100%カバレッジ
- 公開APIは80%以上カバレッジ
- インフラ層は結合テストでカバー

**必須テスト項目**:
```rust
#[cfg(test)]
mod tests {
    // 正常系テスト
    #[test]
    fn test_success_case() { }
    
    // 境界値テスト
    #[test]
    fn test_boundary_case() { }
    
    // 異常系テスト
    #[test]
    #[should_panic(expected = "error message")]
    fn test_error_case() { }
}
```

---

## 7. Git/バージョン管理ガードレール

### 7.1 コミットメッセージ

**フォーマット**:
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
```
feat(onsen): 泉質自動生成機能を追加

OnsenQualityのto_string()メソッドで、
環境省の泉質分類基準に従った日本語泉質名を自動生成する

Closes #123
```

### 7.2 ブランチ戦略

**ブランチ命名**:
```
feature/<issue-number>-<description>
fix/<issue-number>-<description>
refactor/<description>
docs/<description>
```

**例**:
```
feature/123-add-search-function
fix/456-jwt-expiration-bug
refactor/clean-repository-layer
docs/update-api-spec
```

### 7.3 マージルール

**チェックリスト**:
- [ ] すべてのテストがパス
- [ ] コードレビュー完了
- [ ] コンフリクト解消済み
- [ ] マイグレーションファイル確認
- [ ] 環境変数の追加がある場合はREADME更新

---

## 8. CI/CDガードレール

### 8.1 必須チェック項目

**ビルド前**:
```bash
# フォーマットチェック
cargo fmt --check

# Lintチェック
cargo clippy -- -D warnings

# テスト実行
cargo test

# セキュリティ監査
cargo audit
```

### 8.2 デプロイ前チェック

**チェックリスト**:
- [ ] すべてのマイグレーションが適用可能
- [ ] 環境変数が設定済み
- [ ] データベース接続確認
- [ ] ヘルスチェックエンドポイント確認

### 8.3 ロールバック計画

**準備事項**:
1. マイグレーションのdown.sql確認
2. 前バージョンのDockerイメージ保持
3. ロールバック手順書の作成

---

## 9. AI生成コードレビューチェックリスト

### 9.1 構造チェック

- [ ] レイヤー分離が適切か
- [ ] ビルダーパターンが使用されているか
- [ ] Request/Responseモデルが分離されているか

### 9.2 セキュリティチェック

- [ ] パスワードがハッシュ化されているか
- [ ] SQLインジェクション対策がされているか
- [ ] 認証が必要なエンドポイントにガードが設定されているか
- [ ] シークレット情報がハードコーディングされていないか

### 9.3 コード品質チェック

- [ ] 命名規則に従っているか
- [ ] unwrap()が使用されていないか
- [ ] 公開APIにドキュメントコメントがあるか
- [ ] テストコードが追加されているか

### 9.4 データベースチェック

- [ ] マイグレーションファイルが正しく作成されているか
- [ ] 外部キー制約が設定されているか
- [ ] インデックスが適切に設定されているか
- [ ] down.sqlでロールバック可能か

### 9.5 API設計チェック

- [ ] RESTful原則に従っているか
- [ ] エラーハンドリングが適切か
- [ ] レスポンスフォーマットが統一されているか
- [ ] HTTPステータスコードが適切か

---

## 10. 自動化ツール

### 10.1 pre-commitフック

**`.git/hooks/pre-commit`**:
```bash
#!/bin/bash

echo "Running pre-commit checks..."

# フォーマットチェック
cargo fmt --check
if [ $? -ne 0 ]; then
    echo "❌ Format check failed. Run 'cargo fmt' to fix."
    exit 1
fi

# Clippy
cargo clippy -- -D warnings
if [ $? -ne 0 ]; then
    echo "❌ Clippy check failed."
    exit 1
fi

# テスト
cargo test
if [ $? -ne 0 ]; then
    echo "❌ Tests failed."
    exit 1
fi

echo "✅ All checks passed!"
```

### 10.2 GitHub Actions

**`.github/workflows/ci.yml`** (参考):
```yaml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Format check
        run: cargo fmt --check
      
      - name: Clippy
        run: cargo clippy -- -D warnings
      
      - name: Test
        run: cargo test
      
      - name: Security audit
        run: cargo audit
```

---

## 11. ドキュメント管理ガードレール

### 11.1 必須ドキュメント

- `README.md`: プロジェクト概要、セットアップ手順
- `doc/DESIGN.md`: システム設計書
- `doc/AI_GUARDRAILS.md`: このドキュメント
- `doc/API.md`: API仕様書（推奨）

### 11.2 コード変更時のドキュメント更新

**ルール**:
- 新しいエンドポイント追加 → `DESIGN.md`の「5. API設計」を更新
- データベーススキーマ変更 → `DESIGN.md`の「4. データベース設計」を更新
- 新しいエンティティ追加 → `DESIGN.md`の「3. ドメインモデル」を更新
- アーキテクチャ変更 → `DESIGN.md`の「2. アーキテクチャ」を更新

---

## 12. パフォーマンスガードレール

### 12.1 クエリ最適化

**ルール**:
- N+1問題を回避（JOIN使用）
- 必要なカラムのみ取得
- ページネーション実装（大量データ取得時）

**実装例**:
```rust
// ✅ OK: JOINで一括取得
onsen::table
    .left_join(chemicals::table)
    .select((onsen::all_columns, chemicals::all_columns.nullable()))
    .load::<(Onsen, Option<DieselChemical>)>(&mut conn)

// ❌ NG: N+1問題
let onsens = get_all_onsens();
for onsen in onsens {
    let chemical = get_chemical(onsen.chemical_id);  // 毎回クエリ発行
}
```

### 12.2 接続プール

**ルール**:
- データベース接続はプーリング
- 接続数の上限設定
- タイムアウト設定

---

## 13. モニタリング・ロギング

### 13.1 ログレベル

**使い分け**:
- `ERROR`: システムエラー、即対応が必要
- `WARN`: 警告、将来的に問題になる可能性
- `INFO`: 重要な処理の開始/終了
- `DEBUG`: デバッグ情報（開発環境のみ）

**禁止事項**:
```rust
// ❌ NG: 個人情報をログ出力
println!("User email: {}", user.email);
println!("Password: {}", password);

// ✅ OK: 個人情報を含まない
println!("User authenticated successfully");
```

### 13.2 メトリクス

**収集推奨項目**:
- レスポンスタイム
- エラー率
- リクエスト数
- データベース接続数

---

## 14. チェックリスト統合

### 新機能追加時のチェックリスト

- [ ] **アーキテクチャ**: レイヤー分離は適切か
- [ ] **ドメインモデル**: ビルダーパターンを使用しているか
- [ ] **データベース**: マイグレーションファイルを作成したか
- [ ] **API**: Request/Responseモデルを作成したか
- [ ] **認証**: 適切にガードを設定したか
- [ ] **セキュリティ**: シークレット情報をハードコーディングしていないか
- [ ] **テスト**: 単体テストを追加したか
- [ ] **ドキュメント**: DESIGN.mdを更新したか
- [ ] **コミット**: 適切なコミットメッセージを書いたか
- [ ] **CI**: すべてのチェックがパスしたか

---

## 15. AI支援開発のベストプラクティス

### 15.1 プロンプト設計

**効果的なプロンプト例**:
```
以下の要件で温泉エンティティを作成してください：

1. ビルダーパターンを使用
2. 泉質情報を含む
3. OnsenQualityは別の値オブジェクトとして実装
4. 単体テストを含める
5. DESIGN.mdのドメインモデル章に記載されているパターンに従う
```

### 15.2 生成コードのレビューフロー

1. **自動チェック**: CI/CDパイプラインで基本チェック
2. **構造チェック**: レイヤー分離、パターン適用の確認
3. **セキュリティチェック**: 本ドキュメントの「5. セキュリティガードレール」に照らし合わせ
4. **ビジネスロジックチェック**: 要件を満たしているか確認
5. **統合テスト**: 既存コードとの整合性確認

### 15.3 インクリメンタル開発

**原則**:
- 大きな変更を一度に行わない
- 小さな単位でコミット
- 各コミット時にテスト実行
- 段階的にレビュー

---

## 16. まとめ

このガードレールは、AI駆動開発において以下を保証します：

1. **品質**: 一貫したコード品質とアーキテクチャ
2. **セキュリティ**: 基本的なセキュリティ要件の遵守
3. **保守性**: 可読性が高く保守しやすいコード
4. **スケーラビリティ**: 将来の拡張に対応できる設計

定期的にこのドキュメントを見直し、プロジェクトの成長に合わせて更新してください。

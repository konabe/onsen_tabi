# テストカバレッジレポート

## 概要

このドキュメントは、onsen_tabiプロジェクトのテストカバレッジ状況をまとめたものです。

## 実装済みテスト

### 1. Domain層（ドメインロジック）

#### ✅ OnsenQuality（泉質）
- **ファイル**: `src/domain/onsen/onsen_quality.rs`
- **テスト数**: 40+
- **カバレッジ**: ほぼ100%
- **テスト内容**:
  - 単純温泉の生成
  - 各種泉質の日本語名生成
  - 塩化物強塩泉の判定
  - 鉄泉（Ⅱ、Ⅲ価）の判定
  - 放射能泉・弱放射能泉の判定
  - 酸性泉の判定
  - 複数成分の組み合わせ

#### ✅ Chemical（温泉成分）
- **ファイル**: `src/domain/onsen/chemical.rs`
- **テスト数**: 5
- **カバレッジ**: 100%
- **テスト内容**:
  - 各成分の日本語表記変換
  - 鉄イオンの価数表記
  - ラドンの種類判定

#### ✅ OnsenEntity（温泉エンティティ）
- **ファイル**: `src/domain/onsen/onsen_entity.rs`
- **テスト数**: 3
- **カバレッジ**: 高
- **テスト内容**:
  - ビルダーパターンによる生成
  - 必須項目のバリデーション
  - デフォルト値の設定

#### ✅ HotelEntity（宿泊所エンティティ）
- **ファイル**: `src/domain/hotel_entity.rs`
- **テスト数**: 2
- **カバレッジ**: 高
- **テスト内容**:
  - ビルダーパターンによる生成
  - 必須項目のバリデーション

#### ✅ AreaEntity（エリアエンティティ）
- **ファイル**: `src/domain/area_entity.rs`
- **テスト数**: 3
- **カバレッジ**: 高
- **テスト内容**:
  - ビルダーパターンによる生成
  - 必須項目のバリデーション

### 2. Application層（アプリケーションロジック）

#### ✅ API Models
- **ファイル**: `src/application/api_model/*`
- **テスト数**: 10+
- **カバレッジ**: 高
- **テスト内容**:
  - リクエスト/レスポンスのシリアライゼーション
  - エンティティとの相互変換
  - camelCase命名規則の確認

#### ✅ Authentication - Crypto（パスワードハッシュ化）
- **ファイル**: `src/application/auth/crypto_test.rs`
- **テスト数**: 9
- **カバレッジ**: 100%
- **テスト内容**:
  - ハッシュ生成（異なるソルト）
  - パスワード検証（正常系）
  - パスワード検証（異常系）
  - 空パスワードの処理
  - 特殊文字・Unicode文字の処理
  - 長いパスワードの処理
  - 無効なハッシュ形式の処理
  - Argon2フォーマットの確認

#### ✅ Authentication - JWT（トークン管理）
- **ファイル**: `src/application/auth/jwt_test.rs`
- **テスト数**: 13
- **カバレッジ**: 100%
- **テスト内容**:
  - JWT生成
  - JWTデコード（正常系）
  - JWTデコード（異常系）
  - 空・不正なトークンの処理
  - 有効期限の確認（24時間）
  - メールアドレスの保持
  - 発行時刻の確認
  - 異なるシークレットキーでの検証失敗
  - Claims構造の確認

#### ✅ User API Models（認証API）
- **ファイル**: `src/application/api_model/user_api_model.rs`
- **テスト数**: 11
- **カバレッジ**: 100%
- **テスト内容**:
  - AuthRequestのデシリアライゼーション
  - AuthResponseのシリアライゼーション
  - camelCase命名規則
  - 空値・特殊文字の処理
  - 必須フィールドの検証

### 3. Infrastructure層（インフラストラクチャ）

#### ✅ DieselChemical（DBモデル）
- **ファイル**: `src/infrastructure/mysql/diesel_model/diesel_chemical.rs`
- **テスト数**: 11
- **カバレッジ**: 高
- **テスト内容**:
  - OnsenQualityからの変換
  - OnsenQualityへの変換
  - 塩化物強塩泉の処理
  - 鉄泉の種類処理
  - 弱放射能泉の処理
  - 往復変換の整合性確認

---

## テスト不足の領域

### ⚠️ 要対応

#### 1. Controller層
- **ファイル**: `src/application/controller/*_controller.rs`
- **現状**: テストなし
- **必要なテスト**:
  - エンドポイントの統合テスト
  - 認証・認可のテスト
  - エラーハンドリングのテスト
  - HTTPステータスコードの確認

#### 2. RequestGuard（認証ガード）
- **ファイル**: `src/application/controller/request_guard.rs`
- **現状**: テストなし
- **必要なテスト**:
  - Authorizationヘッダーの検証
  - Bearerトークンの解析
  - JWT有効期限の検証
  - ユーザー存在確認

#### 3. Repository層
- **ファイル**: `src/infrastructure/repository/*_repository.rs`
- **現状**: テストなし
- **必要なテスト**:
  - データベースCRUD操作
  - トランザクション処理
  - エラーハンドリング
  - JOIN操作の正確性

### ℹ️ 検討事項

#### 4. DieselOnsen（DBモデル）
- **ファイル**: `src/infrastructure/mysql/diesel_model/diesel_onsen.rs`
- **現状**: テストなし
- **推奨テスト**:
  - エンティティとの相互変換
  - LEFT JOINの処理

---

## テスト実行方法

### すべてのテストを実行
```bash
make test
# または
cargo test
```

### 特定のモジュールのテストを実行
```bash
cargo test --lib domain::onsen::onsen_quality
cargo test --lib application::auth
```

### カバレッジレポートの生成
```bash
make test-coverage
# または
cargo tarpaulin --out Html --output-dir coverage
```

生成されたレポートは `coverage/index.html` で確認できます。

---

## テスト追加のガイドライン

### 1. ドメイン層のテスト
- ビジネスロジックは100%カバレッジを目指す
- 境界値テストを含める
- エラーケースを必ずテスト

### 2. Application層のテスト
- 公開APIは80%以上カバレッジ
- シリアライゼーション/デシリアライゼーションを確認
- バリデーションロジックをテスト

### 3. Infrastructure層のテスト
- 統合テストでカバー
- モックを使用した単体テストも検討
- データベース操作の正確性を確認

### 4. Controller層のテスト
- Rocketのテストクライアントを使用
- エンドポイント毎にテスト
- 認証・認可のテストを含める

---

## CI/CDでのテスト

GitHub Actionsで以下が自動実行されます：

```yaml
- cargo fmt --check    # フォーマットチェック
- cargo clippy         # Lintチェック
- cargo test          # テスト実行
- cargo audit         # セキュリティ監査
```

---

## 次のステップ

### 優先度：高
1. ✅ Authentication（Crypto, JWT）のテスト追加 - **完了**
2. ✅ User API Modelsのテスト追加 - **完了**
3. ✅ DieselChemicalのテスト追加 - **完了**
4. ⏳ Controller層の統合テスト追加
5. ⏳ RequestGuardのテスト追加

### 優先度：中
6. ⏳ Repository層のテスト追加
7. ⏳ DieselOnsenのテスト追加

### 優先度：低
8. E2Eテストの追加
9. パフォーマンステストの追加
10. ロードテストの追加

---

## まとめ

### 現在の状況
- **ドメイン層**: ✅ 優秀（ほぼ100%カバレッジ）
- **Application層**: 🟡 良好（主要部分はカバー済み、Controllerが未対応）
- **Infrastructure層**: 🟡 改善の余地あり（DBモデルの一部のみ）

### 追加されたテスト（このセッション）
- ✅ Crypto（パスワードハッシュ化）: 9テスト
- ✅ JWT（トークン管理）: 13テスト
- ✅ User API Models: 11テスト
- ✅ DieselChemical: 11テスト

**合計追加**: 44テスト

### 推奨事項
1. Controller層の統合テストを優先的に追加
2. RequestGuardの単体テストを追加
3. Repository層はDBを使った統合テストを検討
4. カバレッジレポートを定期的に確認し、80%以上を維持

---

最終更新: 2025-11-07

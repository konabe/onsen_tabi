# onsen_tabi 設計書

## 1. システム概要

### 1.1 目的
温泉を起点とした旅の記録を管理するWebアプリケーションのバックエンドシステム。

### 1.2 技術スタック
- **言語**: Rust
- **Webフレームワーク**: Rocket
- **ORM**: Diesel
- **データベース**: MySQL (RDS)
- **認証**: JWT (jsonwebtoken)
- **パスワードハッシュ**: Argon2
- **インフラ**: AWS (ALB, EC2, RDS), CDK
- **コンテナ**: Docker

### 1.3 システム構成
```
vercel (frontend) <-> Route53 <-> ALB <-> EC2 (Docker) <-> RDS for MySQL
```

---

## 2. アーキテクチャ

### 2.1 レイヤー構成

```
src/
├── application/        # アプリケーション層
│   ├── api_model/     # リクエスト/レスポンスモデル
│   ├── auth/          # 認証・認可
│   └── controller/    # エンドポイント定義
├── domain/            # ドメイン層
│   ├── area_entity.rs
│   ├── hotel_entity.rs
│   └── onsen/         # 温泉ドメイン
├── infrastructure/    # インフラ層
│   ├── mysql/         # DB接続・モデル
│   └── repository/    # データアクセス
└── schema.rs          # DBスキーマ定義
```

### 2.2 依存関係
- Application層 → Domain層 → Infrastructure層
- 各層は下位層のみに依存（Clean Architecture準拠）

---

## 3. ドメインモデル

### 3.1 主要エンティティ

#### 3.1.1 AreaEntity (温泉エリア)
```rust
pub struct AreaEntity {
    pub id: u32,
    pub name: String,           // エリア名
    pub kana: String,           // カナ表記
    pub prefecture: String,     // 都道府県
    pub national_resort: bool,  // 国民保養地か
    pub village: Option<String>,// 温泉郷名
    pub url: String,
    pub description: String,
    pub access: String,
    pub onsens: Vec<OnsenEntity>
}
```

**ビルダーパターン**: `AreaEntityBuilder`

#### 3.1.2 HotelEntity (宿泊所)
```rust
pub struct HotelEntity {
    pub id: u32,
    pub name: String,
    pub has_washitsu: bool,     // 和室有無
    pub solo_available: bool,   // 一人泊可能か
    pub url: String,
    pub description: String,
    pub onsens: Vec<OnsenEntity>
}
```

**ビルダーパターン**: `HotelEntityBuilder`

#### 3.1.3 OnsenEntity (温泉)
```rust
pub struct OnsenEntity {
    pub id: u32,
    pub name: String,
    pub quality: Option<OnsenQuality>,      // 泉質
    pub spring_quality: String,             // 泉質文字列
    pub liquid: Option<SpringLiquid>,       // 液性
    pub osmotic_pressure: Option<SpringOsmoticPressure>,
    pub temperature: Option<SpringTemperature>,
    pub form: SpringForm,                   // 内湯/外湯
    pub is_day_use: bool,                   // 日帰り可能か
    pub url: String,
    pub img_url: Option<String>,
    pub description: String,
    pub area_id: Option<u32>
}
```

**ビルダーパターン**: `OnsenEntityBuilder`

### 3.2 値オブジェクト

#### 3.2.1 OnsenQuality (泉質)
```rust
pub struct OnsenQuality {
    liquid: Option<SpringLiquid>,    // 液性
    pub cations: Vec<Chemical>,      // 陽イオン
    pub anions: Vec<Chemical>,       // 陰イオン
    pub inclusions: Vec<Chemical>    // 含有成分
}
```

**主要メソッド**:
- `new(chemicals: &[Chemical], liquid: Option<SpringLiquid>) -> Self`
- `to_string() -> String` - 日本語泉質名生成
- `is_strong_na_cl() -> bool` - 塩化物強塩泉判定
- `fe_type() -> &str` - 鉄イオン種類
- `is_weak_rn() -> bool` - 弱放射能泉判定

#### 3.2.2 Chemical (温泉成分)
```rust
pub enum Chemical {
    NaIon,              // ナトリウムイオン
    CaIon,              // カルシウムイオン
    MgIon,              // マグネシウムイオン
    ClIon(ClType),      // 塩化物イオン
    HCO3Ion,            // 炭酸水素イオン
    SO4Ion,             // 硫酸塩イオン
    CO2,                // 二酸化炭素
    FeIon(FeType),      // 鉄イオン
    AlIon,              // アルミニウムイオン
    CuIon,              // 銅イオン
    HIon,               // 水素イオン
    IIon,               // よう素
    S,                  // 硫黄
    Rn(RnType)          // ラドン
}
```

**サブタイプ**:
- `ClType`: Normal / Strong
- `FeType`: Normal / Two / Three
- `RnType`: Normal / Weak

#### 3.2.3 SpringLiquid (液性)
```rust
pub enum SpringLiquid {
    Acidic,         // 酸性
    MildlyAcidic,   // 弱酸性
    Neutral,        // 中性
    MildlyAlkaline, // 弱アルカリ性
    Alkaline        // アルカリ性
}
```

#### 3.2.4 SpringForm (営業形態)
```rust
pub enum SpringForm {
    Uchiyu,  // 内湯
    Sotoyu   // 外湯
}
```

---

## 4. データベース設計

### 4.1 テーブル構成

#### 4.1.1 area (温泉エリア)
| カラム名 | 型 | 制約 | 説明 |
|---------|------|------|------|
| id | UNSIGNED INT | PK | ID |
| name | VARCHAR(255) | NOT NULL | エリア名 |
| kana | VARCHAR(255) | NOT NULL | カナ |
| prefecture | VARCHAR(255) | NOT NULL | 都道府県 |
| national_resort | BOOL | NOT NULL | 国民保養地 |
| village | VARCHAR(255) | NULL | 温泉郷名 |
| url | VARCHAR(255) | NOT NULL | URL |
| description | TEXT | NOT NULL | 説明 |
| access | TEXT | NOT NULL | アクセス |

#### 4.1.2 hotel (宿泊所)
| カラム名 | 型 | 制約 | 説明 |
|---------|------|------|------|
| id | UNSIGNED INT | PK | ID |
| name | VARCHAR(255) | NOT NULL | 宿泊所名 |
| has_washitsu | BOOL | NOT NULL | 和室有無 |
| solo_available | BOOL | NOT NULL | 一人泊可 |
| url | VARCHAR(255) | NOT NULL | URL |
| description | TEXT | NOT NULL | 説明 |
| area_id | UNSIGNED INT | FK NULL | エリアID |

#### 4.1.3 onsen (温泉)
| カラム名 | 型 | 制約 | 説明 |
|---------|------|------|------|
| id | UNSIGNED INT | PK | ID |
| name | VARCHAR(255) | NOT NULL | 温泉名 |
| spring_quality | VARCHAR(255) | NOT NULL | 泉質文字列 |
| liquid | VARCHAR(255) | NULL | 液性 |
| osmotic_pressure | VARCHAR(255) | NULL | 浸透圧 |
| temperature | VARCHAR(255) | NULL | 温度 |
| category | VARCHAR(255) | NOT NULL | 形態 |
| day_use | BOOL | NOT NULL | 日帰り可 |
| url | VARCHAR(255) | NOT NULL | URL |
| img_url | VARCHAR(255) | NULL | 画像URL |
| description | TEXT | NOT NULL | 説明 |
| chemical_id | UNSIGNED INT | FK NULL | 成分ID |
| hotel_id | UNSIGNED INT | FK NULL | 宿泊所ID |
| area_id | UNSIGNED INT | FK NULL | エリアID |

#### 4.1.4 chemicals (温泉成分)
| カラム名 | 型 | 制約 | 説明 |
|---------|------|------|------|
| id | UNSIGNED INT | PK | ID |
| na_ion | UNSIGNED INT | NOT NULL | Na+ |
| ca_ion | UNSIGNED INT | NOT NULL | Ca2+ |
| mg_ion | UNSIGNED INT | NOT NULL | Mg2+ |
| cl_ion | UNSIGNED INT | NOT NULL | Cl- |
| hco3_ion | UNSIGNED INT | NOT NULL | HCO3- |
| so4_ion | UNSIGNED INT | NOT NULL | SO42- |
| co2_ion | UNSIGNED INT | NOT NULL | CO2 |
| fe_ion | UNSIGNED INT | NOT NULL | Fe |
| al_ion | UNSIGNED INT | NOT NULL | Al3+ |
| cu_ion | UNSIGNED INT | NOT NULL | Cu2+ |
| h_ion | UNSIGNED INT | NOT NULL | H+ |
| i_ion | UNSIGNED INT | NOT NULL | I |
| s | UNSIGNED INT | NOT NULL | S |
| rn | UNSIGNED INT | NOT NULL | Rn |
| strong_na_cl | BOOL | NOT NULL | 強塩 |
| fe_type | VARCHAR(255) | NOT NULL | Fe種類 |
| weak_rn | BOOL | NOT NULL | 弱放射能 |

#### 4.1.5 user (ユーザー)
| カラム名 | 型 | 制約 | 説明 |
|---------|------|------|------|
| id | UNSIGNED INT | PK | ID |
| email | VARCHAR(255) | NOT NULL UNIQUE | メール |
| hashed_password | VARCHAR(255) | NOT NULL | パスワード |
| role | VARCHAR(255) | NOT NULL | 権限 |

### 4.2 リレーション
- `hotel.area_id` → `area.id`
- `onsen.area_id` → `area.id`
- `onsen.hotel_id` → `hotel.id`
- `onsen.chemical_id` → `chemicals.id`

---

## 5. API設計

### 5.1 エンドポイント一覧

#### 5.1.1 エリア関連
| メソッド | パス | 説明 | 認証 |
|---------|------|------|------|
| GET | `/area` | エリア一覧取得 | 不要 |
| GET | `/area/<id>` | エリア詳細取得 | 不要 |
| POST | `/area` | エリア作成 | 必要 |
| PUT | `/area/<id>` | エリア更新 | 必要 |

#### 5.1.2 宿泊所関連
| メソッド | パス | 説明 | 認証 |
|---------|------|------|------|
| GET | `/hotel?area_id=<id>` | 宿泊所一覧取得 | 不要 |
| GET | `/hotel/<id>` | 宿泊所詳細取得 | 不要 |
| POST | `/hotel` | 宿泊所作成 | 必要 |
| PUT | `/hotel/<id>` | 宿泊所更新 | 必要 |

#### 5.1.3 温泉関連
| メソッド | パス | 説明 | 認証 |
|---------|------|------|------|
| GET | `/onsen?area_id=<id>&hotel_id=<id>` | 温泉一覧取得 | 不要 |
| GET | `/onsen/<id>` | 温泉詳細取得 | 不要 |
| POST | `/onsen` | 温泉作成 | 必要 |
| PUT | `/onsen/<id>` | 温泉更新 | 必要 |

#### 5.1.4 認証関連
| メソッド | パス | 説明 |
|---------|------|------|
| POST | `/signup` | ユーザー登録 |
| POST | `/signin` | ログイン |

### 5.2 リクエスト/レスポンス例

#### 5.2.1 温泉作成 (POST /onsen)

**リクエスト**: `OnsenRequest`
```json
{
  "name": "元禄の湯",
  "chemicals": {
    "naIon": 2,
    "caIon": 1,
    "mgIon": 0,
    "clIon": 5,
    "hco3Ion": 4,
    "so4Ion": 0,
    "co2Ion": 0,
    "feIon": 7,
    "alIon": 0,
    "cuIon": 0,
    "hIon": 0,
    "iIon": 0,
    "s": 0,
    "rn": 0,
    "isStrongNaCl": false,
    "feType": "Two",
    "isWeakRn": false
  },
  "otherSpringQuality": "温泉法の温泉",
  "osmoticPressure": "hypotonic",
  "liquid": "neutral",
  "temperature": "hot",
  "form": "uchiyu",
  "isDayUse": true,
  "url": "https://example.com",
  "imgUrl": "https://example.com/img.png",
  "description": "説明文",
  "areaId": 1
}
```

**レスポンス**: `OnsenResponse`
```json
{
  "id": 1,
  "name": "元禄の湯",
  "quality": {
    "name": "含鉄（Ⅱ）－カルシウム・ナトリウム－炭酸水素塩・塩化物泉",
    "chemicals": ["NaIon", "CaIon", "HCO3Ion", "ClIon", "FeIon"],
    "isStrongNaCl": false,
    "feType": "Two",
    "isWeakRn": false
  },
  "otherSpringQuality": "温泉法の温泉",
  "liquid": "neutral",
  "osmoticPressure": "hypotonic",
  "temperature": "hot",
  "form": "uchiyu",
  "isDayUse": true,
  "url": "https://example.com",
  "imgUrl": "https://example.com/img.png",
  "description": "説明文",
  "area": {
    "id": 1,
    "name": "四万"
  }
}
```

---

## 6. 認証・認可

### 6.1 JWT認証

**実装**: `src/application/auth/jwt.rs`

#### 6.1.1 トークン生成
```rust
pub fn encode_jwt(email: &str) -> String {
    // HS256でJWT生成
    // 有効期限: 24時間
}
```

#### 6.1.2 トークン検証
```rust
pub fn decode_jwt(token: &str) -> Option<Claims> {
    // トークン検証・デコード
}
```

#### 6.1.3 Claims構造
```rust
pub struct Claims {
    pub email: String,
    pub iat: i64,  // 発行時刻
    pub exp: i64   // 有効期限
}
```

### 6.2 パスワードハッシュ化

**実装**: `src/application/auth/crypto.rs`

```rust
pub fn create_hash(plain_password: &str) -> String {
    // Argon2iでハッシュ化
}

pub fn verify_hash(plain_password: &str, hashed_password: &str) -> bool {
    // ハッシュ検証
}
```

### 6.3 リクエストガード

**実装**: `src/application/controller/request_guard.rs`

```rust
pub struct ValidatedUser {
    pub email: String,
    pub role: String
}
```

認証が必要なエンドポイントでは `ValidatedUser` をパラメータに指定:
```rust
#[post("/onsen", data = "<request>")]
pub fn post_onsen(
    _user: ValidatedUser,  // 認証必須
    request: Json<OnsenRequest>
) -> Json<OnsenResponse>
```

---

## 7. データアクセス層

### 7.1 リポジトリパターン

#### 7.1.1 area_repository
**ファイル**: `src/infrastructure/repository/area_repository.rs`

- `get_areas() -> Vec<AreaEntity>`
- `get_area(id: u32) -> Option<AreaEntity>`
- `post_area(area_entity: AreaEntity) -> AreaEntity`
- `put_area(area_entity: AreaEntity) -> ()`

#### 7.1.2 hotel_repository
**ファイル**: `src/infrastructure/repository/hotel_repository.rs`

- `get_hotels(area_id: Option<u32>) -> Vec<HotelEntity>`
- `get_hotel(id: u32) -> Option<HotelEntity>`
- `post_hotel(hotel_entity: HotelEntity) -> HotelEntity`
- `put_hotel(hotel_entity: HotelEntity) -> ()`

#### 7.1.3 onsen_repository
**ファイル**: `src/infrastructure/repository/onsen_repository.rs`

- `get_onsens(area_id: Option<u32>, hotel_id: Option<u32>) -> Vec<OnsenEntity>`
- `get_onsen(id: u32) -> Option<OnsenEntity>`
- `post_onsen(onsen_entity: OnsenEntity) -> OnsenEntity`
- `put_onsen(onsen_entity: OnsenEntity) -> ()`

### 7.2 Dieselモデル

#### 7.2.1 DieselChemical
**ファイル**: `src/infrastructure/mysql/diesel_model/diesel_chemical.rs`

- `OnsenQuality` → `DieselChemical` の相互変換
- 温泉成分データのDB永続化

#### 7.2.2 DieselOnsen
**ファイル**: `src/infrastructure/mysql/diesel_model/diesel_onsen.rs`

- `OnsenEntity` → `Onsen` の相互変換
- LEFT JOINで `DieselChemical` を結合

---

## 8. ビジネスロジック

### 8.1 泉質名自動生成

**実装**: `OnsenQuality::to_string()`

#### アルゴリズム
1. 単純温泉判定（成分なし）
2. 陽イオン・陰イオン・含有成分を分類
3. 日本語表記に変換
4. 環境省基準に従った命名規則で結合

#### 例
```rust
let quality = OnsenQuality::new(
    &vec![NaIon, CaIon, HCO3Ion, ClIon(ClType::Normal), FeIon(FeType::Two)],
    None
);
assert_eq!(
    quality.to_string(),
    "含鉄（Ⅱ）－カルシウム・ナトリウム－炭酸水素塩・塩化物泉"
);
```

### 8.2 泉質分類ロジック

#### 8.2.1 単純温泉
- 陽イオン・陰イオンなし
- 液性のみ表示（例: "弱アルカリ性単純温泉"）

#### 8.2.2 塩化物強塩泉
- `ClType::Strong` を含む
- 他の陰イオンより優先表示

#### 8.2.3 鉄泉
- `FeType`: Normal / Two / Three
- 価数表記（Ⅱ、Ⅲ）

#### 8.2.4 放射能泉
- `RnType`: Normal / Weak
- 弱放射能泉の判定

#### 8.2.5 酸性泉
- `HIon` 含有時は液性が `Acidic` 必須（アサーション）

---

## 9. デプロイ

### 9.1 Docker化

**Dockerfile**: `Dockerfile`

#### ビルドステージ
```dockerfile
FROM amazonlinux:2023 AS build-env
RUN yum install mysql-community-devel
RUN cargo build --release
```

#### デプロイステージ
```dockerfile
FROM amazonlinux:2023
COPY --from=build-env /app/target/release/onsen_tabi /onsen_tabi
RUN diesel migration run
CMD ["/startup.sh"]
```

### 9.2 起動スクリプト

**`startup.sh`**: `startup.sh`
```bash
#!/bin/sh
$HOME/.cargo/bin/diesel migration run
/onsen_tabi
```

### 9.3 AWS CDK

**実装**: `cdk/lib/cdk-stack.ts`

- VPC / ALB / EC2 / RDS の構築
- Auto Scaling Group
- SSL証明書設定

---

## 10. テスト戦略

### 10.1 カバレッジ
- Codecov連携
- 主要ドメインロジックは100%カバー

### 10.2 主要テストケース

#### 10.2.1 泉質生成テスト
**ファイル**: `src/domain/onsen/onsen_quality.rs`

- 単純温泉
- ナトリウム-塩化物泉
- 塩化物強塩泉
- 鉄泉（Ⅱ、Ⅲ価）
- 放射能泉（弱放射能）
- 複数含有成分の組み合わせ

#### 10.2.2 エンティティビルダーテスト
- 必須項目チェック
- 不正データでのパニック確認

#### 10.2.3 APIリクエスト/レスポンステスト
- シリアライズ/デシリアライズ
- エンティティ変換

---

## 11. CORS設定

**実装**: `src/main.rs`

```rust
pub struct CORS;

impl Fairing for CORS {
    fn on_response(&self, _request: &Request, response: &mut Response) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, PUT, PATCH, GET, DELETE"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}
```

---

## 12. 環境変数

### 必須環境変数
- `DATABASE_URL`: MySQL接続URL
- `JWT_SECRET_KEY`: JWT署名用シークレット

### 設定ファイル
- `.env.sample`: 環境変数テンプレート
- `Rocket.toml`: Rocket設定

---

## 13. マイグレーション

### 実行方法
```bash
diesel migration generate {name}
diesel migration run
```

### マイグレーション一覧
- `2023-07-29-015730_create_hotel`
- `2023-07-29-015734_create_onsen`
- `2023-08-07-123223_update_nullable`
- `2023-08-09-105756_create_area`
- `2023-08-11-113837_create_prefecture`
- `2023-08-13-020223_create_onsen_description`
- `2023-08-15-100105_add_url`
- `2023-08-18-233957_create_user`
- `2023-08-20-045231_add_salt_column`
- `2023-08-20-224034_delete_salt_column`
- `2023-08-24-231138_add_description_column`
- `2023-09-02-084319_add_day_use_column`
- `2023-09-02-120550_update_area`
- `2023-09-13-105206_create_chemicals`
- `2024-02-24-170758_non_null_is_day_use`
- `2024-02-26-125320_add_temperature`
- `2024-02-28-102706_create_onsen_img_url`
- `2024-03-07-145307_update_chemicals_order`
- `2024-03-08-070833_add_quality_chemicals`
- `2024-03-26-153317_add_fe_option`
- `2024-03-28-231816_add_solo_available`
- `2024-04-09-224053_add_access_to_area`
- `2024-06-30-070347_add_kana_to_area`

---

## 14. 今後の拡張性

### 14.1 対応可能な機能
- 宿泊履歴管理
- 評価・レビュー機能
- 画像アップロード（S3連携）
- 検索機能強化（全文検索）

### 14.2 スケーラビリティ
- Auto Scaling対応済み
- RDSマスター/スレーブ構成への移行可能
- キャッシュ層追加（Redis）検討可能

---

## 15. 参考資料
- [環境省温泉分類基準](https://www.env.go.jp/nature/onsen/pdf/2-5_p_16.pdf)
- [Rocket公式ドキュメント](https://rocket.rs/)
- [Diesel公式ドキュメント](https://diesel.rs/)

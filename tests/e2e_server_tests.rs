// 実サーバーE2Eテスト
//
// Note: このファイルは実際のWebサーバーに接続してテストを行います。
// テスト実行前にサーバーを起動しておく必要があります。
//
// サーバー起動方法:
//   cargo run &
//   sleep 5  # サーバーが起動するまで待機
//   cargo test --test e2e_server_tests -- --ignored --test-threads=1
//
// Important:
// - これらのテストは実際のデータベース接続が必要です
// - サーバーが起動していない場合、テストは失敗します
// - テストはシーケンシャルに実行してください（--test-threads=1）

use reqwest::blocking::Client;
use serde_json::json;
use std::time::Duration;

/// テスト用のHTTPクライアント設定
fn create_test_client() -> Client {
    Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("Failed to create HTTP client")
}

/// サーバーのベースURL
/// 環境変数 TEST_SERVER_URL で上書き可能（デフォルト: http://localhost:8000）
fn get_base_url() -> String {
    std::env::var("TEST_SERVER_URL").unwrap_or_else(|_| "http://localhost:8000".to_string())
}

/// サーバーが起動しているか確認
fn check_server_health(client: &Client, base_url: &str) -> bool {
    match client.get(format!("{}/", base_url)).send() {
        Ok(response) => response.status().is_success() || response.status().as_u16() == 404,
        Err(_) => false,
    }
}

#[cfg(test)]
mod e2e_server_tests {
    use super::*;

    /// E2Eテスト: サーバー起動確認
    #[test]
    #[ignore] // サーバー起動が必要
    fn test_server_is_running() {
        let client = create_test_client();
        let base_url = get_base_url();

        println!("=== E2E Server Test: Server Health Check ===");
        println!("Testing server at: {}", base_url);

        let is_healthy = check_server_health(&client, &base_url);
        assert!(
            is_healthy,
            "サーバーが起動していません。先にサーバーを起動してください: cargo run"
        );
    }

    /// E2Eテスト: エリア一覧取得
    #[test]
    #[ignore] // サーバー起動が必要
    fn test_get_areas_from_server() {
        let client = create_test_client();
        let base_url = get_base_url();

        println!("=== E2E Server Test: Get Areas ===");

        // サーバー起動確認
        assert!(
            check_server_health(&client, &base_url),
            "サーバーが起動していません"
        );

        // GET /area リクエスト
        let response = client
            .get(format!("{}/area", base_url))
            .send()
            .expect("Failed to send request");

        println!("Status: {}", response.status());
        assert_eq!(
            response.status(),
            200,
            "ステータスコードが200ではありません"
        );

        // レスポンスボディをJSON配列としてパース
        let areas: serde_json::Value = response.json().expect("Failed to parse JSON");
        println!(
            "Response: {}",
            serde_json::to_string_pretty(&areas).unwrap()
        );

        assert!(areas.is_array(), "レスポンスが配列ではありません");
    }

    /// E2Eテスト: ホテル一覧取得
    #[test]
    #[ignore] // サーバー起動が必要
    fn test_get_hotels_from_server() {
        let client = create_test_client();
        let base_url = get_base_url();

        println!("=== E2E Server Test: Get Hotels ===");

        assert!(
            check_server_health(&client, &base_url),
            "サーバーが起動していません"
        );

        let response = client
            .get(format!("{}/hotel", base_url))
            .send()
            .expect("Failed to send request");

        println!("Status: {}", response.status());
        assert_eq!(response.status(), 200);

        let hotels: serde_json::Value = response.json().expect("Failed to parse JSON");
        println!(
            "Response: {}",
            serde_json::to_string_pretty(&hotels).unwrap()
        );

        assert!(hotels.is_array());
    }

    /// E2Eテスト: 温泉一覧取得
    #[test]
    #[ignore] // サーバー起動が必要
    fn test_get_onsens_from_server() {
        let client = create_test_client();
        let base_url = get_base_url();

        println!("=== E2E Server Test: Get Onsens ===");

        assert!(
            check_server_health(&client, &base_url),
            "サーバーが起動していません"
        );

        let response = client
            .get(format!("{}/onsen", base_url))
            .send()
            .expect("Failed to send request");

        println!("Status: {}", response.status());
        assert_eq!(response.status(), 200);

        let onsens: serde_json::Value = response.json().expect("Failed to parse JSON");
        println!(
            "Response: {}",
            serde_json::to_string_pretty(&onsens).unwrap()
        );

        assert!(onsens.is_array());
    }

    /// E2Eテスト: ユーザー登録とログインのフロー
    #[test]
    #[ignore] // サーバー起動が必要
    fn test_user_registration_and_login_flow() {
        let client = create_test_client();
        let base_url = get_base_url();

        println!("=== E2E Server Test: User Registration and Login ===");

        assert!(
            check_server_health(&client, &base_url),
            "サーバーが起動していません"
        );

        let unique_email = format!(
            "e2e_server_user_{}@example.com",
            chrono::Utc::now().timestamp()
        );

        // ステップ1: ユーザー登録
        println!("Step 1: User Registration");
        let signup_response = client
            .post(format!("{}/signup", base_url))
            .json(&json!({
                "email": unique_email,
                "password": "SecurePassword123!"
            }))
            .send()
            .expect("Failed to send signup request");

        println!("Signup Status: {}", signup_response.status());
        assert_eq!(signup_response.status(), 200, "ユーザー登録に失敗しました");

        let signup_body: serde_json::Value = signup_response
            .json()
            .expect("Failed to parse signup response");
        println!(
            "Signup Response: {}",
            serde_json::to_string_pretty(&signup_body).unwrap()
        );

        // JWTトークンを取得
        let token = signup_body["token"]
            .as_str()
            .expect("トークンが取得できませんでした");

        println!("Received token: {}", token);

        // ステップ2: ログイン
        println!("\nStep 2: User Login");
        let signin_response = client
            .post(format!("{}/signin", base_url))
            .json(&json!({
                "email": unique_email,
                "password": "SecurePassword123!"
            }))
            .send()
            .expect("Failed to send signin request");

        println!("Signin Status: {}", signin_response.status());
        assert_eq!(signin_response.status(), 200, "ログインに失敗しました");

        let signin_body: serde_json::Value = signin_response
            .json()
            .expect("Failed to parse signin response");
        println!(
            "Signin Response: {}",
            serde_json::to_string_pretty(&signin_body).unwrap()
        );

        // ログイン後のトークンを確認
        let login_token = signin_body["token"]
            .as_str()
            .expect("ログイントークンが取得できませんでした");

        println!("Login token received: {}", login_token);
    }

    /// E2Eテスト: 認証が必要なエンドポイントへのアクセス
    #[test]
    #[ignore] // サーバー起動が必要
    fn test_authenticated_request() {
        let client = create_test_client();
        let base_url = get_base_url();

        println!("=== E2E Server Test: Authenticated Request ===");

        assert!(
            check_server_health(&client, &base_url),
            "サーバーが起動していません"
        );

        // ステップ1: ユーザー登録してトークン取得
        let unique_email = format!(
            "e2e_auth_user_{}@example.com",
            chrono::Utc::now().timestamp()
        );

        let signup_response = client
            .post(format!("{}/signup", base_url))
            .json(&json!({
                "email": unique_email,
                "password": "TestPassword123!"
            }))
            .send()
            .expect("Failed to send signup request");

        let signup_body: serde_json::Value =
            signup_response.json().expect("Failed to parse response");
        let token = signup_body["token"]
            .as_str()
            .expect("トークンが取得できませんでした");

        println!("Token obtained: {}", token);

        // ステップ2: 認証なしでPOSTリクエスト（失敗するはず）
        println!("\nStep 2: POST without authentication (should fail)");
        let unauthorized_response = client
            .post(format!("{}/area", base_url))
            .json(&json!({
                "name": "テストエリア",
                "kana": "てすとえりあ",
                "prefecture": "テスト県",
                "nationalResort": false,
                "url": "https://example.com",
                "description": "テスト説明",
                "access": "テスト駅から徒歩5分"
            }))
            .send()
            .expect("Failed to send request");

        println!("Unauthorized Status: {}", unauthorized_response.status());
        assert_eq!(
            unauthorized_response.status(),
            401,
            "認証なしのリクエストが拒否されませんでした"
        );

        // ステップ3: 認証ありでPOSTリクエスト（adminロールが必要なため403になる）
        println!("\nStep 3: POST with authentication (should return 403 - admin role required)");
        let authorized_response = client
            .post(format!("{}/area", base_url))
            .header("Authorization", format!("Bearer {}", token))
            .json(&json!({
                "name": format!("E2Eテストエリア_{}", chrono::Utc::now().timestamp()),
                "kana": "いーつーいーてすとえりあ",
                "prefecture": "テスト県",
                "nationalResort": false,
                "url": "https://example.com",
                "description": "E2Eテストで作成",
                "access": "テスト駅から徒歩10分"
            }))
            .send()
            .expect("Failed to send authenticated request");

        println!("Authorized Status: {}", authorized_response.status());
        assert_eq!(
            authorized_response.status(),
            403,
            "一般ユーザーはエリアを作成できません（adminロールが必要）"
        );

        println!("Test passed: Regular user correctly denied access (admin role required)");
    }

    /// E2Eテスト: CORSヘッダーの確認
    #[test]
    #[ignore] // サーバー起動が必要
    fn test_cors_headers() {
        let client = create_test_client();
        let base_url = get_base_url();

        println!("=== E2E Server Test: CORS Headers ===");

        assert!(
            check_server_health(&client, &base_url),
            "サーバーが起動していません"
        );

        // OPTIONSリクエストを送信
        let response = client
            .request(reqwest::Method::OPTIONS, format!("{}/area", base_url))
            .send()
            .expect("Failed to send OPTIONS request");

        println!("OPTIONS Status: {}", response.status());

        // CORSヘッダーを確認
        let headers = response.headers();

        if let Some(cors_origin) = headers.get("access-control-allow-origin") {
            println!(
                "Access-Control-Allow-Origin: {}",
                cors_origin.to_str().unwrap()
            );
            assert_eq!(cors_origin, "*", "CORS Originが正しく設定されていません");
        }

        if let Some(cors_methods) = headers.get("access-control-allow-methods") {
            println!(
                "Access-Control-Allow-Methods: {}",
                cors_methods.to_str().unwrap()
            );
        }

        println!("CORS headers are properly configured");
    }

    /// E2Eテスト: エラーハンドリング
    #[test]
    #[ignore] // サーバー起動が必要
    fn test_error_handling() {
        let client = create_test_client();
        let base_url = get_base_url();

        println!("=== E2E Server Test: Error Handling ===");

        assert!(
            check_server_health(&client, &base_url),
            "サーバーが起動していません"
        );

        // 存在しないエンドポイント
        println!("Testing non-existent endpoint");
        let response = client
            .get(format!("{}/nonexistent", base_url))
            .send()
            .expect("Failed to send request");

        println!("Status: {}", response.status());
        assert_eq!(
            response.status(),
            404,
            "存在しないエンドポイントで404が返されませんでした"
        );

        // 不正なJSON
        println!("\nTesting invalid JSON");
        let response = client
            .post(format!("{}/signin", base_url))
            .header("Content-Type", "application/json")
            .body("{invalid json}")
            .send()
            .expect("Failed to send request");

        println!("Status: {}", response.status());
        assert_eq!(
            response.status(),
            400,
            "不正なJSONで400が返されませんでした"
        );
    }
}

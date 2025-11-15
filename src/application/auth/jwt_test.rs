#[cfg(test)]
mod tests {
    use super::super::{decode_jwt_with_secret, encode_jwt_with_secret, Claims};
    use chrono::Utc;

    const TEST_SECRET: &str = "test_secret_key_for_testing_purposes_only";

    #[test]
    fn test_encode_jwt_creates_valid_token() {
        let email = "test@example.com";
        let token = encode_jwt_with_secret(email, TEST_SECRET);

        assert!(!token.is_empty());
        assert!(token.contains('.'));
        let parts: Vec<&str> = token.split('.').collect();
        assert_eq!(parts.len(), 3); // header.payload.signature
    }

    #[test]
    fn test_decode_jwt_with_valid_token() {
        let email = "test@example.com";
        let token = encode_jwt_with_secret(email, TEST_SECRET);

        let claims = decode_jwt_with_secret(&token, TEST_SECRET);
        assert!(claims.is_some());

        let claims = claims.unwrap();
        assert_eq!(claims.email, email);
        assert!(claims.iat > 0);
        assert!(claims.exp > claims.iat);
    }

    #[test]
    fn test_decode_jwt_with_invalid_token() {
        let invalid_token = "invalid.token.string";

        let claims = decode_jwt_with_secret(invalid_token, TEST_SECRET);
        assert!(claims.is_none());
    }

    #[test]
    fn test_decode_jwt_with_empty_token() {
        let empty_token = "";

        let claims = decode_jwt_with_secret(empty_token, TEST_SECRET);
        assert!(claims.is_none());
    }

    #[test]
    fn test_decode_jwt_with_malformed_token() {
        let malformed_token = "not_a_jwt_at_all";

        let claims = decode_jwt_with_secret(malformed_token, TEST_SECRET);
        assert!(claims.is_none());
    }

    #[test]
    fn test_jwt_expiration_is_24_hours() {
        let email = "test@example.com";
        let token = encode_jwt_with_secret(email, TEST_SECRET);

        let claims = decode_jwt_with_secret(&token, TEST_SECRET).unwrap();
        let duration = claims.exp - claims.iat;

        // 24時間 = 86400秒
        assert_eq!(duration, 86400);
    }

    #[test]
    fn test_jwt_contains_correct_email() {
        let test_emails = vec![
            "user@example.com",
            "admin@test.jp",
            "test.user+tag@domain.co.uk",
        ];

        for email in test_emails {
            let token = encode_jwt_with_secret(email, TEST_SECRET);
            let claims = decode_jwt_with_secret(&token, TEST_SECRET).unwrap();
            assert_eq!(claims.email, email);
        }
    }

    #[test]
    fn test_jwt_iat_is_recent() {
        let email = "test@example.com";
        let now_before = Utc::now().timestamp();
        let token = encode_jwt_with_secret(email, TEST_SECRET);
        let now_after = Utc::now().timestamp();

        let claims = decode_jwt_with_secret(&token, TEST_SECRET).unwrap();

        // iatは現在時刻の前後数秒以内
        assert!(claims.iat >= now_before);
        assert!(claims.iat <= now_after + 1);
    }

    #[test]
    fn test_different_tokens_for_same_email() {
        let email = "test@example.com";

        let token1 = encode_jwt_with_secret(email, TEST_SECRET);
        std::thread::sleep(std::time::Duration::from_millis(1001)); // 1秒待つ
        let token2 = encode_jwt_with_secret(email, TEST_SECRET);

        // 異なる発行時刻により異なるトークンが生成される
        assert_ne!(token1, token2);
    }

    #[test]
    fn test_jwt_with_special_characters_in_email() {
        let email = "test+special@example.com";
        let token = encode_jwt_with_secret(email, TEST_SECRET);

        let claims = decode_jwt_with_secret(&token, TEST_SECRET).unwrap();
        assert_eq!(claims.email, email);
    }

    #[test]
    fn test_decode_jwt_with_wrong_secret() {
        let email = "test@example.com";
        let token = encode_jwt_with_secret(email, TEST_SECRET);

        // 異なるシークレットキーでデコード
        let different_secret = "different_secret_key";

        // デコードは失敗するはず
        let claims = decode_jwt_with_secret(&token, different_secret);
        assert!(claims.is_none());
    }

    #[test]
    fn test_claims_structure() {
        let claims = Claims {
            email: "test@example.com".to_string(),
            iat: 1234567890,
            exp: 1234654290,
        };

        assert_eq!(claims.email, "test@example.com");
        assert_eq!(claims.iat, 1234567890);
        assert_eq!(claims.exp, 1234654290);
    }
}

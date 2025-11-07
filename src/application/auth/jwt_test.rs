#[cfg(test)]
mod tests {
    use super::super::{encode_jwt, decode_jwt, Claims};
    use chrono::Utc;
    use std::env;

    fn setup_test_env() {
        env::set_var("JWT_SECRET_KEY", "test_secret_key_for_testing_purposes_only");
    }

    #[test]
    fn test_encode_jwt_creates_valid_token() {
        setup_test_env();
        let email = "test@example.com";
        let token = encode_jwt(email);
        
        assert!(!token.is_empty());
        assert!(token.contains('.'));
        let parts: Vec<&str> = token.split('.').collect();
        assert_eq!(parts.len(), 3); // header.payload.signature
    }

    #[test]
    fn test_decode_jwt_with_valid_token() {
        setup_test_env();
        let email = "test@example.com";
        let token = encode_jwt(email);
        
        let claims = decode_jwt(&token);
        assert!(claims.is_some());
        
        let claims = claims.unwrap();
        assert_eq!(claims.email, email);
        assert!(claims.iat > 0);
        assert!(claims.exp > claims.iat);
    }

    #[test]
    fn test_decode_jwt_with_invalid_token() {
        setup_test_env();
        let invalid_token = "invalid.token.string";
        
        let claims = decode_jwt(invalid_token);
        assert!(claims.is_none());
    }

    #[test]
    fn test_decode_jwt_with_empty_token() {
        setup_test_env();
        let empty_token = "";
        
        let claims = decode_jwt(empty_token);
        assert!(claims.is_none());
    }

    #[test]
    fn test_decode_jwt_with_malformed_token() {
        setup_test_env();
        let malformed_token = "not_a_jwt_at_all";
        
        let claims = decode_jwt(malformed_token);
        assert!(claims.is_none());
    }

    #[test]
    fn test_jwt_expiration_is_24_hours() {
        setup_test_env();
        let email = "test@example.com";
        let token = encode_jwt(email);
        
        let claims = decode_jwt(&token).unwrap();
        let duration = claims.exp - claims.iat;
        
        // 24時間 = 86400秒
        assert_eq!(duration, 86400);
    }

    #[test]
    fn test_jwt_contains_correct_email() {
        setup_test_env();
        let test_emails = vec![
            "user@example.com",
            "admin@test.jp",
            "test.user+tag@domain.co.uk",
        ];
        
        for email in test_emails {
            let token = encode_jwt(email);
            let claims = decode_jwt(&token).unwrap();
            assert_eq!(claims.email, email);
        }
    }

    #[test]
    fn test_jwt_iat_is_recent() {
        setup_test_env();
        let email = "test@example.com";
        let now_before = Utc::now().timestamp();
        let token = encode_jwt(email);
        let now_after = Utc::now().timestamp();
        
        let claims = decode_jwt(&token).unwrap();
        
        // iatは現在時刻の前後数秒以内
        assert!(claims.iat >= now_before);
        assert!(claims.iat <= now_after + 1);
    }

    #[test]
    fn test_different_tokens_for_same_email() {
        setup_test_env();
        let email = "test@example.com";
        
        let token1 = encode_jwt(email);
        std::thread::sleep(std::time::Duration::from_millis(1001)); // 1秒待つ
        let token2 = encode_jwt(email);
        
        // 異なる発行時刻により異なるトークンが生成される
        assert_ne!(token1, token2);
    }

    #[test]
    fn test_jwt_with_special_characters_in_email() {
        setup_test_env();
        let email = "test+special@example.com";
        let token = encode_jwt(email);
        
        let claims = decode_jwt(&token).unwrap();
        assert_eq!(claims.email, email);
    }

    #[test]
    fn test_decode_jwt_with_wrong_secret() {
        setup_test_env();
        let email = "test@example.com";
        let token = encode_jwt(email);
        
        // 異なるシークレットキーを設定
        env::set_var("JWT_SECRET_KEY", "different_secret_key");
        
        // デコードは失敗するはず
        let claims = decode_jwt(&token);
        assert!(claims.is_none());
        
        // 元に戻す
        setup_test_env();
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

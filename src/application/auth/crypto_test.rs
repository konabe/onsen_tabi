#[cfg(test)]
mod tests {
    use super::super::{create_hash, verify_hash};

    #[test]
    fn test_create_hash_generates_different_hashes() {
        let password = "test_password123";
        let hash1 = create_hash(password);
        let hash2 = create_hash(password);
        
        // 同じパスワードでも異なるソルトにより異なるハッシュが生成される
        assert_ne!(hash1, hash2);
        assert!(hash1.starts_with("$argon2"));
        assert!(hash2.starts_with("$argon2"));
    }

    #[test]
    fn test_verify_hash_with_correct_password() {
        let password = "correct_password";
        let hashed = create_hash(password);
        
        assert!(verify_hash(password, &hashed));
    }

    #[test]
    fn test_verify_hash_with_incorrect_password() {
        let password = "correct_password";
        let wrong_password = "wrong_password";
        let hashed = create_hash(password);
        
        assert!(!verify_hash(wrong_password, &hashed));
    }

    #[test]
    fn test_verify_hash_with_empty_password() {
        let password = "";
        let hashed = create_hash(password);
        
        assert!(verify_hash(password, &hashed));
        assert!(!verify_hash("not_empty", &hashed));
    }

    #[test]
    fn test_verify_hash_with_special_characters() {
        let password = "p@ssw0rd!#$%^&*()";
        let hashed = create_hash(password);
        
        assert!(verify_hash(password, &hashed));
        assert!(!verify_hash("p@ssw0rd", &hashed));
    }

    #[test]
    fn test_verify_hash_with_unicode_characters() {
        let password = "パスワード日本語123";
        let hashed = create_hash(password);
        
        assert!(verify_hash(password, &hashed));
        assert!(!verify_hash("パスワード", &hashed));
    }

    #[test]
    fn test_verify_hash_with_long_password() {
        let password = "a".repeat(1000);
        let hashed = create_hash(&password);
        
        assert!(verify_hash(&password, &hashed));
        assert!(!verify_hash(&"a".repeat(999), &hashed));
    }

    #[test]
    fn test_verify_hash_with_invalid_hash_format() {
        let password = "test_password";
        let invalid_hash = "invalid_hash_format";
        
        // 無効なハッシュ形式の場合はfalseを返す
        assert!(!verify_hash(password, invalid_hash));
    }

    #[test]
    fn test_hash_format() {
        let password = "test123";
        let hashed = create_hash(password);
        
        // Argon2ハッシュの基本フォーマット確認
        assert!(hashed.starts_with("$argon2i$"));
        assert!(hashed.len() > 50); // ハッシュは十分な長さがある
    }
}

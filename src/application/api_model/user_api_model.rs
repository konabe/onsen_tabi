use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthResponse {
    pub token: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_auth_request_deserialization() {
        let json = r#"{"email":"test@example.com","password":"password123"}"#;
        let request: AuthRequest = serde_json::from_str(json).unwrap();
        
        assert_eq!(request.email, "test@example.com");
        assert_eq!(request.password, "password123");
    }

    #[test]
    fn test_auth_request_deserialization_camel_case() {
        let json = r#"{"email":"user@test.com","password":"pass"}"#;
        let request: AuthRequest = serde_json::from_str(json).unwrap();
        
        assert_eq!(request.email, "user@test.com");
        assert_eq!(request.password, "pass");
    }

    #[test]
    fn test_auth_request_with_empty_values() {
        let json = r#"{"email":"","password":""}"#;
        let request: AuthRequest = serde_json::from_str(json).unwrap();
        
        assert_eq!(request.email, "");
        assert_eq!(request.password, "");
    }

    #[test]
    fn test_auth_request_with_special_characters() {
        let json = r#"{"email":"test+tag@example.com","password":"p@ssw0rd!#$%"}"#;
        let request: AuthRequest = serde_json::from_str(json).unwrap();
        
        assert_eq!(request.email, "test+tag@example.com");
        assert_eq!(request.password, "p@ssw0rd!#$%");
    }

    #[test]
    fn test_auth_response_serialization() {
        let response = AuthResponse {
            token: "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9".to_string(),
        };
        let json = serde_json::to_string(&response).unwrap();
        
        assert!(json.contains("token"));
        assert!(json.contains("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9"));
    }

    #[test]
    fn test_auth_response_serialization_camel_case() {
        let response = AuthResponse {
            token: "test_token".to_string(),
        };
        let json = serde_json::to_string(&response).unwrap();
        
        // camelCaseで出力されることを確認
        assert!(json.contains("\"token\""));
    }

    #[test]
    fn test_auth_response_with_empty_token() {
        let response = AuthResponse {
            token: "".to_string(),
        };
        let json = serde_json::to_string(&response).unwrap();
        
        assert_eq!(json, r#"{"token":""}"#);
    }

    #[test]
    fn test_auth_response_with_long_token() {
        let long_token = "a".repeat(1000);
        let response = AuthResponse {
            token: long_token.clone(),
        };
        let json = serde_json::to_string(&response).unwrap();
        
        assert!(json.contains(&long_token));
    }

    #[test]
    #[should_panic]
    fn test_auth_request_missing_email_field() {
        let json = r#"{"password":"password123"}"#;
        let _request: AuthRequest = serde_json::from_str(json).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_auth_request_missing_password_field() {
        let json = r#"{"email":"test@example.com"}"#;
        let _request: AuthRequest = serde_json::from_str(json).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_auth_request_invalid_json() {
        let json = r#"{"email":"test@example.com","password":}"#;
        let _request: AuthRequest = serde_json::from_str(json).unwrap();
    }
}

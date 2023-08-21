use argon2::{Config, Variant, Version};
use rand::{distributions::Alphanumeric, Rng};

pub fn create_hash(plain_password: &str) -> String {
    let salt: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();
    let config = Config {
        variant: Variant::Argon2i,
        version: Version::Version13,
        mem_cost: 65536,
        time_cost: 1,
        lanes: 4,
        secret: &[],
        ad: &[],
        hash_length: 32,
    };
    argon2::hash_encoded(plain_password.as_bytes(), &salt.as_bytes(), &config).unwrap()
}

pub fn verify_hash(plain_password: &str, hashed_password: &str) -> bool {
    let matches =
        argon2::verify_encoded(hashed_password, plain_password.as_bytes()).unwrap_or(false);
    return matches;
}

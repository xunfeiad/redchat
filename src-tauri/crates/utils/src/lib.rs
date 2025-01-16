use rand::distributions::{Alphanumeric, DistString};
use sha2::{Digest, Sha256};
pub mod email;

pub fn hash_password(password: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password);
    format!("{:x}", hasher.finalize())
}

pub fn get_random_str() -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), 6)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hash_password() {
        let result = hash_password("xunfei");
        assert_eq!(
            result,
            "f9566788443daf0670d86a9fc1f3ac5019cb27c502c257f3a0f2815e6f0a7d46"
        );
    }
}

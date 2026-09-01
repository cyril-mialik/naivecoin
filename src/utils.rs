use sha2::{Digest, Sha256};

pub const DIFFICULTY: usize = 4;

pub fn hash_data(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);

    let result = hasher.finalize();
    hex::encode(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_data() {
        let hash = hash_data("hello");

        assert_eq!(hash.len(), 64);
        assert_eq!(
            hash,
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );
    }

    #[test]
    fn test_hash_deterministic() {
        let data = "test data";

        assert_eq!(hash_data(data), hash_data(data));
    }
}

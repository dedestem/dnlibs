#[cfg(test)]
mod sha256_tests {
    use crate::sha256;

    #[test]
    fn test_sha256_hash() {
        let value = b"HashThisYouHasher";
        let hash = sha256::hash_sha256(value);

        // Convert the hash to hex string
        let hashhex: String = hash.iter().map(|byte| format!("{:02x}", byte)).collect();

        // Expected correct hash
        assert_eq!(
            hashhex, "a795e8927999f942719664fe940131b317cd3b03c80fa24131ce4702834a3178",
            "SHA-256 hash mismatch"
        );
    }

    #[test]
    fn test_sha256_hash_empty() {
        let value = b"";
        let hash = sha256::hash_sha256(value);

        // This is the SHA-256 hash of an empty string.
        let expected_hash = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
        let hashhex: String = hash.iter().map(|byte| format!("{:02x}", byte)).collect();

        assert_eq!(hashhex, expected_hash, "Empty input hash mismatch");
    }
}

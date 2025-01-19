#[cfg(test)]
mod sha256_files_tests {
    use crate::sha256_files;

    #[test]
    fn test_sha256_files_hash() {
        let path = "Readme.md";

        let hash = sha256_files::hash_sha256_file(path).expect("Failed to compute SHA-256 hash");

        // Convert the hash (which is [u8; 32]) to a hex string, byte by byte
        let hashhex: String = hash.iter().map(|&byte| format!("{:02x}", byte)).collect();

        let expected_hash = "bd3ecbe5a84be0c10e63169bbb90dc1cac3750e7bf2df9ba908b707dff15c35e";
        
        // Compare with expected hash
        assert_eq!(hashhex, expected_hash, "SHA-256 hash mismatch");
    }
}

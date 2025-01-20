#[cfg(test)]
mod toml_tests {
    use crate::toml;

    #[test]
    fn test_write_and_read_toml() {
        // Schrijf een waarde naar de TOML
        if let Err(e) = toml::write_value("test.toml", "new_key", "new_value") {
            panic!("Error writing value: {}", e);
        }

        // Lees de waarde uit en controleer of deze klopt
        match toml::read_value("test.toml", "new_key") {
            Ok(Some(value)) => assert_eq!(value, "new_value"),
            Ok(None) => panic!("Key not found in TOML"),
            Err(e) => panic!("Error reading value: {}", e),
        }

        // Opruimen
        if let Err(e) = std::fs::remove_file("test.toml") {
            eprintln!("Failed to remove test file: {}", e);
        }
    }
}

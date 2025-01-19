use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, Write};

/// Read a value for a key (vab) from a TOML file
pub fn read_value(file_path: &str, key: &str) -> io::Result<Option<String>> {
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    for line in content.lines() {
        if let Some((k, v)) = line.split_once('=') {
            let k = k.trim();
            let v = v.trim().trim_matches('"'); // Remove quotes around string values
            if k == key {
                return Ok(Some(v.to_string()));
            }
        }
    }

    Ok(None)
}

/// Write a value for a key (vab) into a TOML file (overwrites if exists)
pub fn write_value(file_path: &str, key: &str, value: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .read(true) // Allow reading the file
        .write(true) // Allow writing to the file
        .create(true) // Create the file if it doesn't exist
        .truncate(false) // Do not wipe the file each write
        .open(file_path)?;

    // Read the existing content of the file
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    // Check if the key already exists in the file
    let mut key_found = false;
    let mut new_content = String::new();

    for line in content.lines() {
        if line.starts_with(&format!("{} = ", key)) {
            // Overwrite the line with the new value
            new_content.push_str(&format!("{} = \"{}\"\n", key, value));
            key_found = true;
        } else {
            // Keep the line as it is
            new_content.push_str(line);
            new_content.push('\n');
        }
    }

    // If the key wasn't found, append the new key-value pair
    if !key_found {
        new_content.push_str(&format!("\n{} = \"{}\"", key, value));
    }

    // Now overwrite the file with the new content
    file.set_len(0)?; // Clear the file
    file.seek(std::io::SeekFrom::Start(0))?; // Move cursor to the beginning
    file.write_all(new_content.as_bytes())?;

    Ok(())
}

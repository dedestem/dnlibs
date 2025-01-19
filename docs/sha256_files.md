# sha256_files

> [!IMPORTANT]
>
> 🦀 Uses Rust STD
> 🧩 Uses an internal dependency

<br>

### 🧩 Internal dependencies list

 - [sha256](sha256.md "sha256.md")

<br>

### Example
``` rust
use dnlibs::sha256_files::sha256file;

fn main() {
    let path = "example.txt";

    match sha256file(path) {
        Ok(hash) => {
            println!("SHA-256 hash of {}:", path);
            for byte in hash {
                print!("{:02x}", byte);
            }
            println!();
        }
        Err(e) => {
            eprintln!("Something wrent wrong reading the file {}", e);
        }
    }
}
```

<br>

### Use case(s)
 - Filehash

<br>

[Back](index.md "index.md")

# sha256_files

> [!IMPORTANT]
>
> 🦀 Uses Rust STD
> 🧩 Uses an internal dependency

<br>

### 🧩 Internal dependencies list

 - [sha256](sha256.md "sha256.md")

<br>

### 🦀 STD's

 - [fs](https://doc.rust-lang.org/std/fs/ "https://doc.rust-lang.org/std/fs/")
 - [io](https://doc.rust-lang.org/stable/std/io/ "https://doc.rust-lang.org/stable/std/io/")

<br>

### Example
``` rust
use dnlibs::sha256_files::hash_sha256_file;

fn main() {
    let path = "example.txt";

    match hash_sha256_file(path) {
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

#### ⏱ Tests
 - test_sha256_files_hash


<br>

#### Notes
use ``` sha256sum filename ``` on linux to verify / update test hash!



<br>

[Back](index.md "index.md")

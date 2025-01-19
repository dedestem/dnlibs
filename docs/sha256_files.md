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

> [!WARNING]  
> Example lib paths incorrect! (Example created in lib itself!)

``` rust
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
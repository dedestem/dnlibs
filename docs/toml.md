# toml

> [!CAUTION]
> 🚧 Code + Doc still under construction! 🚧

<br>

> [!IMPORTANT]
>
> 🦀 Uses Rust STD


### Writing to the toml

```Rust
// Writing a value to the TOML file
match utoml::write_value("test.toml", "new_key", "new_value") {
    Ok(_) => {}
    Err(e) => eprintln!("Error writing value: {}", e),
} 
```
### Reading from the toml

```rust
// Reading a value from the TOML file
let value = match utoml::read_value("test.toml", "new_key") {
    Ok(Some(value)) => value,
    Ok(None) => return,
    Err(e) => {
        eprint!("Error reading value: {}", e);
        process::exit(1);
    }
};

println!("{}", value);
```

##### ---------------------------------------------------
[Back](index.md "index.md")
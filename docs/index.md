# DNLibs docs

##### Include DNLibs in Cargo.toml

``` toml
[package]
name = "untitled"
version = "0.1.0"
edition = "2021"

[dependencies] 
dnlibs = { git = "https://github.com/dedestem/dnlibs.git" }                         # Way 1 - If you want the newest version
dnlibs = { git = "https://github.com/dedestem/dnlibs.git", branch = "main" }        # Way 2 - If you want the newst version of an specfic branch
dnlibs = { git = "https://github.com/dedestem/dnlibs.git", rev = "commit_hash" }    # Way 3 - (RECOMMENDED) - If you want an specfic version

```

### ------------------------------------------------------
#### Cryptographic

[sha256](sha256.md "sha256.md")

[sha256_files](sha256_files.md "sha256_files.md")

#### Enviroment

[time](time.md "time.md")

#### Parsers

[toml](toml.md "toml.md")

### ------------------------------------------------------

[Back](../readme.md "../readme.md")

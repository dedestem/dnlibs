# sha256_files

> [!IMPORTANT]
>
> 🦀 Uses Rust STD

<br>

### 🦀 STD's

 - [time](https://doc.rust-lang.org/std/time/ "https://doc.rust-lang.org/std/time/")

<br>

### Examples
```rust
use dnlibs::time;
//+++
```


#### Is leap year
```rust
time::is_leap_year(2010);
```
```rust
println!("{}", time::is_leap_year(2025));
```
Result: ```false```

<br>

#### Current time
```rust
time::get_time()
```
```rust
println!("Current time: {}", time::get_time());
```
Result: ```Current time: 13:28:46```

<br>

#### Current date
```rust
time::get_date()
```
```rust
println!("Current time: {}", time::get_date());
```
Result: ```Current date: 2025-01-12```

<br>

#### Current time & date
```rust
time::get_time_and_date()
```
```rust
println!("Current date and time: {}", time::get_time_and_date());
```
Result: ```Current date and time: 2025-01-12 13:28:46```

<br>

### Use case(s)
 - What year is it?

<br>

#### ⏱ Tests
 - test_leapyear_true
 - test_leapyear_false


<br>

[Back](index.md "index.md")

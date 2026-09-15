# Error Handling (Result and Option)

This lesson explains how Rust represents and handles errors using `Option` and `Result`, how to match on them, and how to propagate errors with `?`.

---

## 1) `Option<T>` recap

`Option<T>` represents an optional value: either `Some(value)` or `None`.

```rust
fn find_first_even(nums: &[i32]) -> Option<i32> {
    for &n in nums {
        if n % 2 == 0 {
            return Some(n);
        }
    }
    None
}

fn main() {
    let v = vec![1, 3, 4];
    match find_first_even(&v) {
        Some(n) => println!("first even = {}", n),
        None => println!("no even number"),
    }
}
```

### Exercises

1. Write a function `first_positive(nums: &[i32]) -> Option<i32>` that returns the first positive number or `None`.
2. Call it and `match` on the result to print either the value or `"none"`.

---

## 2) `Result<T, E>` basics

`Result<T, E>` expresses success (`Ok(T)`) or failure (`Err(E)`). Choose an error type (commonly `String` or a custom enum).

```rust
fn divide(n: f64, d: f64) -> Result<f64, String> {
    if d == 0.0 {
        Err(String::from("division by zero"))
    } else {
        Ok(n / d)
    }
}

fn main() {
    match divide(10.0, 2.0) {
        Ok(v) => println!("result = {}", v),
        Err(e) => println!("error: {}", e),
    }
}
```

### Exercises

1. Implement `safe_sqrt(x: f64) -> Result<f64, String>` that returns an error on negative input.
2. Call it and handle both `Ok` and `Err`.

---

## 3) Convenience methods

- `unwrap()` — returns the `Ok` or panics on `Err` (useful in quick examples, avoid in production).
- `expect("msg")` — like `unwrap()` but with a custom panic message.
- `map`, `and_then` (a.k.a. `and_then` / `flat_map`) transform `Result`/`Option` values.

```rust
let res: Result<i32, &str> = Ok(2);
let doubled = res.map(|x| x * 2); // Ok(4)
```

---

## 4) Propagating errors with `?`

Use `?` inside functions that return `Result` to return early on error.

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username(path: &str) -> Result<String, io::Error> {
    let mut s = String::new();
    let mut f = File::open(path)?; // if Err -> return Err
    f.read_to_string(&mut s)?;    // if Err -> return Err
    Ok(s)
}
```

This replaces nested `match`/`if let` boilerplate.

### Exercises

1. Rewrite a function that opens a file and reads the first line using `?`.

---

## 5) Custom error types

For libraries or complex programs, define an error `enum` and implement `From` conversions for convenience.

```rust
use std::num::ParseIntError;

#[derive(Debug)]
enum MyError {
    Io(std::io::Error),
    Parse(ParseIntError),
}

impl From<std::io::Error> for MyError {
    fn from(e: std::io::Error) -> Self { MyError::Io(e) }
}

impl From<ParseIntError> for MyError {
    fn from(e: ParseIntError) -> Self { MyError::Parse(e) }
}

fn read_number(path: &str) -> Result<i32, MyError> {
    let s = std::fs::read_to_string(path)?; // io::Error -> MyError via From
    let n: i32 = s.trim().parse()?;         // ParseIntError -> MyError via From
    Ok(n)
}
```

### Exercises

1. Create a small error `enum` for a function that can fail to read a file or parse a number, and use `?` with `From` conversions.

---

## Practice challenge

Write a program that:

- reads a number from a file (path given in `main`)
- parses it into an integer
- prints `Even` or `Odd` depending on the value
- return a `Result<(), YourError>` from `main` and use `?` for error propagation

Hints: use `std::fs::read_to_string`, `trim().parse::<i32>()`, and define a small `enum` for errors or use `Box<dyn std::error::Error>` for convenience.

---

## Suggested next topics

- Working with `thiserror` / `anyhow` crates for ergonomic error types
- Error handling patterns in async code
- Designing recoverable vs non-recoverable errors


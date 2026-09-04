# Control Flow, Functions, and Modules

This next part covers the building blocks you use in almost every Rust program: conditions, loops, reusable functions, and splitting code into modules.

---

## 1) if / else

Rust uses `if` as an expression, so it can return a value.

```rust
let x = 10;

if x > 5 {
    println!("x is bigger than 5");
} else {
    println!("x is not bigger than 5");
}
```

You can also do this:

```rust
let result = if x > 5 { "big" } else { "small" };
println!("{result}");
```

### Exercises

let resulf = if x % 2 { "even" } else { "odd" };
println!("{result}");

if x % 2 == 0 {
println!("even");
} else {
println!("odd");
}

1. Write an `if`/`else` that checks whether a number is even or odd.
2. Use `if` as an expression and print the result.

---

## 2) Loops

Rust has three main loop forms:

- `loop` — runs forever until `break`
- `while` — repeats while a condition is true
- `for` — iterates over ranges or collections

```rust
let mut i = 0;
loop {
    i += 1;
    println!("i = {i}");
    if i == 3 {
        break;
    }
}
```

```rust
let mut x = 0;
while x < 3 {
    println!("x = {x}");
    x += 1;
}
```

```rust
for n in 1..=5 {
    println!("n = {n}");
}
```

### Exercises

let mut i = 0;
loop {
i += 1;
println!(" {i}");
if i == 5 {
break;
}
}

let mut i = 5;
while i <= 5 {
println!(" {i}");
i -= 1;
}

let mut rs = 0;
for i in 1..=10 {
rs += i;
}
println!("{rs}");

1. Use `loop` to print numbers from 1 to 5, then stop.
2. Use `while` to count down from 5 to 1.
3. Use `for` to sum numbers from 1 to 10.

---

## 3) Functions

Functions are declared with `fn`.

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

println!("{}", add(2, 3));
```

A function can also return early with `return`, but the last expression is usually returned automatically.

### Exercises

fn square(x: i32) -> i32 {
x \* x
}

fn is_even(n: i32) -> bool {
n % 2
}

fn greet(name: &str) {
println!("Hello, {name}!");
}

1. Write `fn square(x: i32) -> i32` and return `x * x`.

2. Write `fn is_even(n: i32) -> bool` that returns whether `n` is even.
3. Write `fn greet(name: &str)` that prints `Hello, {name}!`.

---

## 4) Match

`match` is Rust's powerful pattern-matching tool.

```rust
let value = 2;

match value {
    1 => println!("one"),
    2 => println!("two"),
    _ => println!("something else"),
}
```

### Exercises

1. Match on a `u32` and print whether it is small, medium, or large.
2. Match on a `char` and print if it is a vowel.

---

## 5) Modules

Rust allows you to split code into modules.

```rust
mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

fn main() {
    println!("{}", math::add(4, 5));
}
```

Rules:

- `mod` creates a module
- `pub` makes items visible outside the module
- `use` can bring symbols into scope

```rust
use crate::math::add;

println!("{}", add(10, 20));
```

mod math {
    pub fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }
}

fn main() {
    let a: i32 = math::multiply(3,4);
}

mod strings {
    pub fn greeting() {
        println!("hello world");
    }
}
### Exercises

1. Create a `mod math` with `pub fn multiply(a: i32, b: i32) -> i32`.
2. Call it from `main`.
3. Create a second module, such as `mod strings`, and print a greeting from there.

---

## Quick challenge

fn main() {
    println!("input one number");
    input
}
Write a tiny program that:

- asks for a number
- checks if it is even or odd
- prints the numbers from 1 to that number
- calls a function to compute the sum

---

## Suggested next exercises

After you finish the examples above, practice these three ideas:



1. Make a function that returns a boolean.
2. Use `match` with a small enum.
3. Split a program into `mod` blocks and call functions across modules.

When you're comfortable with these, we can move to the next topic: ownership, borrowing, and lifetimes.

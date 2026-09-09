# Structs, Enums, and Pattern Matching

This lesson covers the next important Rust building blocks: structs, enums, and pattern matching.

---

## 1) Structs

A struct groups related data together.

```rust
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let p = Person {
        name: String::from("Alice"),
        age: 30,
    };

    println!("{} {}", p.name, p.age);
}
```

### Exercises

1. Define a `struct Point { x: i32, y: i32 }` and create a point `(3, 4)`.
2. Print `p.x` and `p.y`.
3. Add a method `fn new(x: i32, y: i32) -> Point` inside `impl Point`.

---

## 2) Enums

An enum represents a value that can be one of several variants.

```rust
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let d = Direction::Up;
    println!("{:?}", d);
}
```

### Exercises

1. Define `enum Light { Red, Yellow, Green }`.
2. Create a variable `let light = Light::Red;`.
3. Print which light it is.

---

## 3) Pattern Matching

`match` is Rust's main pattern-matching tool.

```rust
let number = 3;

match number {
    1 => println!("one"),
    2 => println!("two"),
    _ => println!("other"),
}
```

### Exercises

1. Match on a number and print `even` or `odd`.
2. Match on a `Direction` and print the direction name.
3. Match on a `Light` enum and print a message.

---

## 4) Methods

You can define methods with `impl`.

```rust
struct Circle {
    radius: i32,
}

impl Circle {
    fn area(&self) -> i32 {
        self.radius * self.radius * 3
    }
}

fn main() {
    let c = Circle { radius: 5 };
    println!("{}", c.area());
}
```

### Exercises

1. Add a method `fn area(&self) -> i32` to a `Rectangle` struct.
2. Create a `Person` struct with a method `fn greet(&self)`.

---

## 5) Option and Result (preview)

These are common Rust enums.

```rust
let x: Option<i32> = Some(5);
let y: Option<i32> = None;
```

### Exercise

1. Create an `Option<i32>` and match on it.
2. Print `"has value"` if there is one, otherwise `"none"`.

---

## Practice challenge

Write a small program that:

- defines a `struct User`
- defines an enum `Status`
- matches on the enum
- prints a message depending on the status

This helps connect all three topics together.

---

## Suggested next exercises

1. Create a `struct` with two fields and a constructor.
2. Add a `match` expression for an enum.
3. Try `Option` and `Result` in a small example.

After this, the next topic is usually collections and iterators.

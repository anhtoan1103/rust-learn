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


```rust
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

fn main() {
    let p = Point {
        x: 3,
        y: 4
    };
    println!("{}, {}", p.x, p.y);
}
```

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

```rust
enum Light {
    Red,
    Yellow,
    Green,
}

fn main() {
    let light = Light::Red;
    println!("{:?}", light);
}
```

1. Define `enum Light { Red, Yellow, Green }`.
2. Create a variable `let light = Light::Red;`.
3. Print which light it is.

---

## 3) Pattern Matching

`match` is Rust's main pattern-matching tool.

```rust
let number = 3;
match number % 2 {
    0 => println!("even"),
    _ => println!("odd"),
}

let direction = Direction::Right;
match direction {
    Direction::Up => println!("Up"),
    Direction::Down => println!("Down"),
    Direction::Left => println!("Left"),
    Direction::Right => println!("right"),
}

let light = Light::Yellow;
match light {
    Light::Yellow => println!("Yellow"),
    Light::Green => println!("Green"),
    Light::Red => println!("Red"),
}
```

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

```rust
struct Rectangle {
    a: i32,
    b: i32,
}

impl Rectangle {
    fn area(&self) -> i32 {
        self.a * self.b
    }
}

struct Person {
    name: String,
}

impl Person {
    fn greet(&self) {
        println!("hello");
    }
}
```

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

```rust
let length: Option<i32> = Some(5);
match length {
    Some(_) => println!("has value"),
    None => println!("none"),
}
```

1. Create an `Option<i32>` and match on it.
2. Print `"has value"` if there is one, otherwise `"none"`.

---

## Practice challenge

Write a small program that:

```rust
struct User {
    name: String,
}

enum Status {
    Good,
    Error,
    Warning,
}

fn main() {
    let user = User {
        name: String::from("Alice"),
    };

    let status = Status::Good;

    match status {
        Status::Good => println!("{} is good", user.name),
        Status::Error => println!("{} has an error", user.name),
        Status::Warning => println!("{} has a warning", user.name),
    }
}
```

- defines a `struct User`
- defines an enum `Status`
- matches on the enum
- prints a message depending on the status

This helps connect all three topics together.

---

## Suggested next exercises

```rust
struct Animal {
    name: String,
    age: i32,
}

impl Animal {
    fn new() -> Self {
        Self {
            name: String::from("toto"),
            age: 0,
        }
    }
}

enum LogLevel {
    Error,
    Warning,
    Info,
    Debug,
}

fn main() {
    let loglevel = LogLevel::Info;

    match loglevel {
        LogLevel::Error => println!("Error"),
        LogLevel::Warning => println!("Warning"),
        LogLevel::Info => println!("Info"),
        LogLevel::Debug => println!("Debug"),
    }

    let animal = Animal::new();
    println!("{} {}", animal.name, animal.age);

    let result = divide(10.0, 2.0);
    println!("{:?}", result);

    let optional = divide_optional(10.0, 0.0);
    println!("{:?}", optional);
}

fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err(String::from("Cannot divide by zero!"))
    } else {
        Ok(numerator / denominator)
    }
}

fn divide_optional(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}
```

1. Create a `struct` with two fields and a constructor.
2. Add a `match` expression for an enum.
3. Try `Option` and `Result` in a small example.

After this, the next topic is usually collections and iterators.

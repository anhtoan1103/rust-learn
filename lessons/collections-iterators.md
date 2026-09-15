# Collections and Iterators

This lesson introduces the main data structures in Rust: vectors, strings, hash maps, and iterators.

Rust is strict about types, so collections are usually typed and predictable. This makes them powerful and safe.

---

## 1) Vectors

A vector is a growable array.

```rust
fn main() {
    let mut nums = vec![1, 2, 3];
    nums.push(4);

    println!("{:?}", nums);
    println!("first = {}", nums[0]);
}
```

A vector can store values of the same type.

```rust
let numbers: Vec<i32> = vec![10, 20, 30];
```

### Exercises

1. Create a `Vec<i32>` with 3 numbers.
2. Add one more number.
3. Print the vector.

```rust
fn main() {
    let mut nums = vec![1, 2, 3];
    nums.push(4);
    println!("{:?}", nums);
}
```
---

## 2) Strings

Rust has two main string types:

- `String` = owned, growable string
- `&str` = string slice

```rust
fn main() {
    let name = String::from("Alice");
    println!("Hello, {}!", name);

    let greeting = "hello";
    println!("{}", greeting);
}
```

You can push characters and text into a `String`.

```rust
fn main() {
    let mut word = String::from("hi");
    word.push('!');
    println!("{}", word);
}
```

### Exercises

```rust
fn main() {
    let mut city = String::from("Ho Chi Minh");
    city.push('!');
    println!("{}", city);
}
```

1. Create a `String` called `city`.
2. Add some text to it.
3. Print it.

---

## 3) HashMap

`HashMap` stores values by key.

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Alice", 90);
    scores.insert("Bob", 85);

    println!("{:?}", scores);
    println!("Alice score = {}", scores["Alice"]);
}
```

### Exercises

1. Create a `HashMap<&str, i32>`.
2. Insert a few key-value pairs.
3. Print the map.
```rust
use std::collections::HashMap;

fn main() {
    let mut info: HashMap<&str, i32> = HashMap::new();
    info.insert("toto", 20);
    info.insert("totobeo", 26);
    println!("{:?}", info);
}
```
---

## 4) Iterators

An iterator is an object that lets you go through a collection one item at a time.

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4];

    for n in numbers {
        println!("{}", n);
    }
}
```

You can also transform values with methods like `iter()`, `map()`, and `collect()`.

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4];

    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("{:?}", doubled);
}
```

### Exercises

1. Create a vector of numbers.
2. Use `.iter()` and `.sum()` to add them together.
3. Print the total.

```rust
fn main() {
    let nums = vec![1, 2, 3];
    let sum: i32 = nums.iter().sum();
    for n in nums {
        println!("{}", n);
    }
    println!("{}", sum);
}
```

---

## 5) Closures

A closure is a small anonymous function.

```rust
fn main() {
    let add_one = |x: i32| x + 1;
    println!("{}", add_one(5));
}
```

Closures are often used with iterators.

```rust
fn main() {
    let nums = vec![1, 2, 3];
    let result: Vec<i32> = nums.iter().map(|x| x + 1).collect();
    println!("{:?}", result);
}
```

### Exercises

```rust
fn main() {
    let nums = vec![1, 2, 3];
    let doubled: Vec<i32> = nums.iter().map(|x| x * 2).collect();
    println!("{}", doubled);
}
```

1. Create a vector of numbers.
2. Use a closure to double each item.
3. Print the result.

---

## Practice challenge

Write a small program that:
```rust
fn main() {
    let mut nums = vec![1, 2, 3];
    nums.push(4);
    nums.push(5);
    let sum: i32 = nums.iter().sum();
    println!("{}", sum);
}
```
- creates a `Vec<i32>`
- adds a few values
- uses an iterator to sum them
- prints the total

Example (optional hint):

```rust
fn main() {
    let numbers = vec![5, 10, 15, 20];
    let total: i32 = numbers.iter().sum();
    println!("Total = {}", total);
}
```

---

## Suggested next exercises

```rust
use std::collections::HashMap;

fn main() {
    let names = vec!["toan", "toto", "totobeo"];
    for name in &names {
        println!("{}", name);
    }
    let mut map: HashMap<i32, String> = HashMap::new();
    map.insert(1, String::from("toto1"));
    map.extend([
        (2, String::from("toto2")),
        (3, String::from("toto3")),
        (4, String::from("toto4")),
    ]);
    println!("{:?}", map.get(&1));
    let list_name: Vec<String> = names.iter().map(|name| {
        let mut s = name.to_string();
        s.push('t');
        s
    }).collect();
    println!("{:?}", list_name);
}
```


1. Create a `Vec<String>` and print each item.
2. Add entries to a `HashMap` and lookup one by key.
3. Use `.iter().map()` to transform a vector.
4. Try a closure that filters values.

After this, the next topic is often error handling and `Result`/`Option` in more depth.

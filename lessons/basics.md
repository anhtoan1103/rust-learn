# Basics — Theory + Exercises

This lesson teaches Rust basics in a simple, step-by-step way: short theory sections followed by many small exercises you can practice right away.

---

## 1) Variables, mutability, and shadowing (theory)

- A variable in Rust is created with `let`. By default it is immutable (you cannot change it).
- Add `mut` to make it mutable: `let mut x = 5;` lets you change `x` later.
- Shadowing means declaring a new variable with the same name: `let x = x + 1;` creates a new `x` that replaces the old one in scope.

### Exercises

1. Declare an immutable `i32` variable named `a` with value `10`. Try to change it (what happens?).
2. Declare `let mut b = 3;` then add `b += 2;`. Print `b`.
3. Use shadowing: `let s = 5; let s = s + 2;` — what's the final `s`?
4. let a = 10;
5. let mut b = 3;
   let b = b + 2;  
    fmt.print!(b);
6. final s is 7

---

## 2) Primitive types (theory)

- Common types: integers (`i32`, `i64`, `u32`), floats (`f32`, `f64`), `bool`, and `char`.
- Rust infers types but you can annotate: `let n: i64 = 100;`.

### Exercises

1. Create `let n: i64 = 1000;` and `let f: f64 = 2.5;`. Print both.
2. Try `let c: char = '💡';` and print it.
3. let n: i64 = 1000;
   let f: f64 = 2.5;  
    fmt.print!(n);  
    fmt.print!(f);
4. let c: char = '💡';
   fmt.print!(c);

---

## 3) Functions (theory)

- Functions use `fn name(params) -> Type { ... }`.
- Last expression is returned without `return` typically.

### Exercises

1. Write a function `fn add(a: i32, b: i32) -> i32 { ... }` and call `add(2,3)`.
2. Write `fn square(x: i32) -> i32` that returns `x * x` and test it.
3. fn add(a: i32, b: i32) -> i32 {
   a + b  
    }  
    add(2,3);
4. fn square(x: i32) -> i32 {
   x \* x  
    }  
    square(5);

---

## 4) Ownership & borrowing (short theory)

- Each value has one owner. When the owner goes out of scope, the value is dropped.
- Passing a `String` to a function moves ownership unless you pass a reference `&String` (borrowing).

### Exercises

1. Create `let s = String::from("hello");` then call a function `takes(s);` that takes `String`. Try to use `s` after the call (what happens?).
2. Modify the previous by calling `borrows(&s)` where `fn borrows(s: &String) {}` — can you use `s` afterward?

3.

```plaintext
let s = String::from("hello");
takes(s);
fmt.print!(s);
```

9.

```
fn borrows(s: &String) {};
let s = String::from("hello");
borrows(s);
fmt.print!(s);

```

---

## 5) Structs and enums (theory)

- `struct Point { x: i32, y: i32 }` groups data.
- `enum Direction { Up, Down, Left, Right }` is a type with different variants.

### Exercises

1. Define `struct Point { x: i32, y: i32 }`, create a point `(3,4)`, and print `p.x` and `p.y`.
2. Define `enum Light { Red, Yellow, Green }`. Match on a value and print which color it is.

---

## 6) Vectors and iteration (theory)

- `Vec<T>` is a growable array: `let mut v = vec![1,2,3]; v.push(4);`.
- Iterate with `for x in &v { ... }` to borrow elements, or `for x in v { ... }` to take ownership.

### Exercises

1. Create `let mut v = vec![1,2]; v.push(3);` then iterate by reference and print each item.

---

## Answers (check after trying)

1. Compiler error if you try to assign to immutable `a` — Rust prevents reassignment.
2. `b` prints `5`.
3. Final `s` is `7`.
4. Prints `1000` and `2.5`.
5. Prints the bulb emoji `💡`.
6. `add(2,3)` returns `5`.
7. `square(4)` returns `16`.
8. After moving `s` into `takes`, you cannot use `s` (compile error: value moved).
9. Borrowing with `&s` lets you use `s` afterward.
10. `p.x = 3`, `p.y = 4`.
11. Pattern matching prints the matched variant.
12. Printing items shows `1`, `2`, `3`.

---

## How to run examples quickly

Create a small `examples/` file or run `cargo run -- basics` if you kept the earlier example. Otherwise, create a tiny `main.rs` that demonstrates each exercise.

---

Write a one-line note in `progress.md` after you finish these exercises.

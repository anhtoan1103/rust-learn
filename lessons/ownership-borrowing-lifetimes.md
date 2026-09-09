# Ownership, Borrowing, and Lifetimes

Ownership là hệ thống giúp Rust quản lý bộ nhớ mà không cần garbage collector.

---

## 1) Ownership

Mỗi giá trị trong Rust có một owner. Một giá trị chỉ có một owner tại một thời điểm.

Khi owner ra khỏi scope, giá trị được giải phóng tự động.

```rust
fn main() {
    let text = String::from("hello");
    println!("{text}");
} // text được giải phóng ở đây
```

### Move

Các kiểu dữ liệu như `String` được move khi gán hoặc truyền vào hàm:

```rust
let first = String::from("hello");
let second = first;

println!("{second}");
// println!("{first}"); // lỗi: first đã được move
```

`i32`, `bool`, `char` và các kiểu đơn giản khác thường được copy:

```rust
let first = 5;
let second = first;

println!("{first} {second}");
```

### Clone

Nếu cần tạo một bản sao độc lập của `String`, dùng `clone()`:

```rust
let first = String::from("hello");
let second = first.clone();

println!("{first} {second}");
```

---

## 2) Borrowing

Thay vì chuyển ownership, ta có thể mượn một giá trị bằng reference `&`:

```rust
fn print_length(text: &String) {
    println!("{}", text.len());
}

fn main() {
    let text = String::from("hello");
    print_length(&text);
    println!("{text}");
}
```

Hàm có thể nhận `&str` để làm việc với cả `String` và string literal:

```rust
fn greet(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    let name = String::from("Rust");
    greet(&name);
    greet("world");
}
```

### Mutable borrowing

Dùng `&mut` khi muốn thay đổi giá trị được mượn:

```rust
fn add_exclamation(text: &mut String) {
    text.push('!');
}

fn main() {
    let mut text = String::from("Hello");
    add_exclamation(&mut text);
    println!("{text}");
}
```

Tại một thời điểm, Rust chỉ cho phép một mutable reference hoặc nhiều immutable references, không cho phép trộn hai loại này cùng lúc.

---

## 3) Lifetimes

Lifetime cho compiler biết các references còn hợp lệ trong khoảng thời gian nào. Rust thường tự suy luận lifetime nên ta không cần viết chúng.

```rust
fn longer<'a>(first: &'a str, second: &'a str) -> &'a str {
    if first.len() > second.len() {
        first
    } else {
        second
    }
}

fn main() {
    let result = longer("long", "short");
    println!("{result}");
}
```

`'a` là một lifetime parameter. Nó nói rằng reference được trả về phải sống ít nhất lâu bằng các reference đầu vào liên quan.

Lifetime không làm dữ liệu sống lâu hơn; chúng chỉ giúp Rust kiểm tra references có hợp lệ hay không.

---

## Exercises

1. Tạo một `String`, move nó sang biến khác, và quan sát lỗi nếu dùng lại biến cũ.
```rust
let name = String::from("hello");
let age = name;
println!("{}", name);

```
2. Viết hàm nhận `&str` và trả về độ dài của chuỗi.

```rust
pub fn length(i: &str) -> usize {
    i.len()
}
```

3. Viết hàm nhận `&mut String` và thêm một ký tự vào chuỗi.

```rust
pub fn append(text: &mut String) {
    text.push('1');
}
```
4. Viết hàm `longer` trả về chuỗi dài hơn trong hai `&str`.
```rust
pub fn longer(str1: &str, str2: &str) -> &str {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}
```

## Suggested next exercises

1. Viết hàm nhận `String` và trả ownership về caller.
```rust
pub fn owner(text: String) -> String {
    text
}
```
2. Sửa một đoạn code có lỗi borrow bằng cách dùng scope nhỏ hơn.
```rust
let text = String::from("hello");
let text1 = &mut text;
println!("{}", text);

let text = String::from("hello");
{
    let text1 = &mut text;
    text1.push('!');
}

println!("{}", text);
```
3. Tạo một struct chứa reference và thử thêm lifetime annotation.

```rust
struct NameRef<'a> {
    value: &'a str,
}

impl <'a> NameRef<'a> {
    fn as_str(&self) -> &str {
        self.value
    }
}

fn main() {
    let name = String::from("Rust");
    let ref_holder = NameRef { value: &name };
    println!("{}", ref_holder.as_str());
}

Khi đã quen với ownership và borrowing, ta có thể chuyển sang structs, enums và pattern matching.

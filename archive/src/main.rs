use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use chrono::Local;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

type TodoStore = Arc<Mutex<HashMap<u32, String>>>;

fn parse_todo_id(path: &str) -> Option<u32> {
    if path.starts_with("/api/todos/") {
        path.strip_prefix("/api/todos/")
            .and_then(|id_str| id_str.parse::<u32>().ok())
    } else {
        None
    }
}

fn handle_client(mut stream: TcpStream, todos: TodoStore) -> std::io::Result<()> {
    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer)?;
    if n == 0 {
        return Ok(());
    }

    let req = String::from_utf8_lossy(&buffer[..n]);
    let first_line = req.lines().next().unwrap_or("");
    let mut parts = first_line.split_whitespace();
    let method = parts.next().unwrap_or("");
    let path = parts.next().unwrap_or("");

    let (status, content_type, body) = match (method, path) {
        ("GET", "/api/todos") => {
            let store = todos.lock().unwrap();
            let items: Vec<String> = store
                .iter()
                .map(|(id, text)| format!(r#"{{"id":{},"text":"{}"}}"#, id, text))
                .collect();
            let body = format!("[{}]", items.join(","));
            ("200 OK", "application/json", body)
        }
        ("POST", "/api/todos") => {
            let mut store = todos.lock().unwrap();
            let new_id = store.keys().max().unwrap_or(&0) + 1;
            store.insert(new_id, "New Todo".to_string());
            let body = format!(r#"{{"id":{},"text":"New Todo"}}"#, new_id);
            ("201 Created", "application/json", body)
        }
        ("GET", _) if path.starts_with("/api/todos/") => {
            if let Some(id) = parse_todo_id(path) {
                let store = todos.lock().unwrap();
                if let Some(text) = store.get(&id) {
                    let body = format!(r#"{{"id":{},"text":"{}"}}"#, id, text);
                    ("200 OK", "application/json", body)
                } else {
                    ("404 Not Found", "application/json", r#"{"error":"Todo not found"}"#.to_string())
                }
            } else {
                ("400 Bad Request", "application/json", r#"{"error":"Invalid ID"}"#.to_string())
            }
        }
        ("PUT", _) if path.starts_with("/api/todos/") => {
            if let Some(id) = parse_todo_id(path) {
                let mut store = todos.lock().unwrap();
                if store.contains_key(&id) {
                    store.insert(id, "Updated Todo".to_string());
                    let body = format!(r#"{{"id":{},"text":"Updated Todo"}}"#, id);
                    ("200 OK", "application/json", body)
                } else {
                    ("404 Not Found", "application/json", r#"{"error":"Todo not found"}"#.to_string())
                }
            } else {
                ("400 Bad Request", "application/json", r#"{"error":"Invalid ID"}"#.to_string())
            }
        }
        ("DELETE", _) if path.starts_with("/api/todos/") => {
            if let Some(id) = parse_todo_id(path) {
                let mut store = todos.lock().unwrap();
                if store.remove(&id).is_some() {
                    ("204 No Content", "application/json", String::new())
                } else {
                    ("404 Not Found", "application/json", r#"{"error":"Todo not found"}"#.to_string())
                }
            } else {
                ("400 Bad Request", "application/json", r#"{"error":"Invalid ID"}"#.to_string())
            }
        }
        ("GET", "/api/time") => {
            let now = Local::now().format("%Y-%m-%d %H:%M:%S %:z").to_string();
            ("200 OK", "text/plain; charset=utf-8", now)
        }
        ("GET", "/api/info") => {
            let body = r#"{"name":"rust-server","version":"0.1.0"}"#.to_string();
            ("200 OK", "application/json", body)
        }
        _ => ("404 Not Found", "application/json", r#"{"error":"Endpoint not found"}"#.to_string()),
    };

    let response = format!(
        "HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
        status,
        content_type,
        body.len(),
        body
    );

    stream.write_all(response.as_bytes())?;
    stream.flush()?;

    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    let todos: TodoStore = Arc::new(Mutex::new(HashMap::new()));
    println!("Server running at http://127.0.0.1:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let todos_clone = Arc::clone(&todos);
                if let Err(err) = handle_client(stream, todos_clone) {
                    eprintln!("Failed to handle client: {}", err);
                }
            }
            Err(err) => eprintln!("Connection failed: {}", err),
        }
    }

    Ok(())
}

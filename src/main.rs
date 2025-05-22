use clap::Parser;
use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::thread;

#[derive(Parser)]
struct Args {
    /// Directory to serve
    #[arg(short = 'd', long, default_value = ".")]
    path: String,
    
    /// Port to serve on
    #[arg(short = 'p', long, default_value = "8080")]
    port: u16,
    
    /// Host to bind to
    #[arg(long, default_value = "127.0.0.1")]
    host: String,
}

fn main() {
    let args = Args::parse();
    let addr = format!("{}:{}", args.host, args.port);
    
    println!("🚀 Static server starting at http://{}", addr);
    println!("📁 Serving directory: {}", args.path);
    
    let listener = TcpListener::bind(&addr).unwrap();
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let serve_path = args.path.clone();
        
        thread::spawn(move || {
            handle_connection(stream, &serve_path);
        });
    }
}

fn handle_connection(mut stream: TcpStream, serve_path: &str) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    
    let request = String::from_utf8_lossy(&buffer[..]);
    let request_line = request.lines().next().unwrap_or("");
    
    let path = if let Some(path) = request_line.split_whitespace().nth(1) {
        if path == "/" {
            "/index.html"
        } else {
            path
        }
    } else {
        "/index.html"
    };
    
    let file_path = format!("{}{}", serve_path, path);
    let file_path = Path::new(&file_path);
    
    let (status_line, contents) = if file_path.exists() && file_path.is_file() {
        ("HTTP/1.1 200 OK", fs::read_to_string(file_path).unwrap_or_default())
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404 Not Found".to_string())
    };
    
    let content_type = match file_path.extension().and_then(|s| s.to_str()) {
        Some("html") => "text/html",
        Some("css") => "text/css",
        Some("js") => "application/javascript",
        Some("json") => "application/json",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("svg") => "image/svg+xml",
        _ => "text/plain",
    };
    
    let response = format!(
        "{}\r\nContent-Type: {}\r\nContent-Length: {}\r\nAccess-Control-Allow-Origin: *\r\n\r\n{}",
        status_line,
        content_type,
        contents.len(),
        contents
    );
    
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
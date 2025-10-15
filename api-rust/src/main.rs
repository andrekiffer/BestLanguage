use std::io::{self, BufRead, Write, Read};
use std::net::{TcpListener, TcpStream};
use std::thread;

#[derive(serde::Deserialize)]
struct PrimeRequest {
    number: i32,
}

#[derive(serde::Serialize)]
struct PrimeResponse {
    is_prime: bool,
}

fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn handle_client(mut stream: TcpStream) {
    let mut reader = io::BufReader::new(&stream);
    let mut full_request = String::new();
    loop {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {
                full_request.push_str(&line);
                if line.trim().is_empty() {
                    break;
                }
            }
            Err(_) => break,
        }
    }
    
    // Verificar se é POST
    if !full_request.starts_with("POST") {
        send_error_response(&mut stream, "405 Method Not Allowed");
        return;
    }
    
    if !full_request.contains("POST / HTTP/1.1") && !full_request.contains("POST / ") {
        send_error_response(&mut stream, "404 Not Found");
        return;
    }

    let content_length = full_request
        .lines()
        .find(|line| line.to_lowercase().starts_with("content-length:"))
        .and_then(|line| line.split(':').nth(1))
        .and_then(|s| s.trim().parse::<usize>().ok())
        .unwrap_or(0);
    
    let mut body = vec![0; content_length];
    if content_length > 0 {
        if let Err(e) = reader.read_exact(&mut body) {
            println!("Erro ao ler corpo: {}", e);
            send_error_response(&mut stream, "400 Bad Request");
            return;
        }
    }
    
    let body_str = String::from_utf8_lossy(&body);
    
    let prime_request: PrimeRequest = match serde_json::from_str(&body_str) {
        Ok(req) => req,
        Err(e) => {
            println!("Erro ao fazer parse do JSON: {}", e);
            send_error_response(&mut stream, "400 Bad Request");
            return;
        }
    };
    
    // Verificar se é primo
    let is_prime_result = is_prime(prime_request.number);
    
    let response = PrimeResponse {
        is_prime: is_prime_result,
    };
    
    // Enviar resposta
    let json_response = serde_json::to_string(&response).unwrap();
    let http_response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nAccess-Control-Allow-Origin: *\r\n\r\n{}",
        json_response.len(),
        json_response
    );
    
    if let Err(e) = stream.write_all(http_response.as_bytes()) {
        println!("Erro ao enviar resposta: {}", e);
        return;
    }
    
    if let Err(e) = stream.flush() {
        println!("Erro ao fazer flush: {}", e);
    }
}

fn send_error_response(stream: &mut TcpStream, status: &str) {
    let response = format!(
        "HTTP/1.1 {}\r\nContent-Type: application/json\r\nContent-Length: 0\r\n\r\n",
        status
    );
    let _ = stream.write_all(response.as_bytes());
    let _ = stream.flush();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("Servidor iniciado em http://127.0.0.1:3000");
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("Erro ao aceitar conexão: {}", e);
            }
        }
    }
}
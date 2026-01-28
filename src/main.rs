use last_project::ThreadPool;
use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
use reqwest::blocking::Client;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Could not bind to port");
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(200) {
        let stream = stream.expect("Connection failed");

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down server.");
}

pub fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    
    let request_line = match buf_reader.lines().next() {
        Some(Ok(line)) => line,
        _ => 
        {
           println!("Connection closed by the browser."); 
                   return;

        }
    };

    // --- 1. TRANSLATION ROUTE ---
    if request_line.starts_with("GET /translate?word=") {
        // Extract the word between "word=" and the next space
        let word = request_line
            .split("word=")
            .nth(1)
            .unwrap_or("")
            .split_whitespace()
            .next()
            .unwrap_or("");
        let translation = translate(word);
        let body = format!("Original: {}\nTranslation (HY): {}", word, translation);
        

        // Note: Content-Type header is vital for displaying Armenian characters correctly
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain; charset=utf-8\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),   
            body
        );
        let _ = stream.write_all(response.as_bytes());
        return; 
    }

    // --- 2. STATIC FILE ROUTES ---
    let (status_line, filename) = match request_line.as_str() {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    match fs::read_to_string(filename) {
        Ok(contents) => {
            let length = contents.len();
            let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
            let _ = stream.write_all(response.as_bytes());
        }
        Err(_) => {
            // Safety fallback if 404.html is also missing
            let response = "HTTP/1.1 404 NOT FOUND\r\n\r\nPage not found";
            let _ = stream.write_all(response.as_bytes());
        }
    }
}



fn translate(word: &str) -> String {
    let client = Client::new();
    
    // MyMemory API: Free for demos, no key needed, very reliable.
    // Format: get?q=WORD&langpair=SOURCE|TARGET
    let url = format!(
        "https://api.mymemory.translated.net/get?q={}&langpair=en|hy",
        word
    );

    match client.get(url).send() {
        Ok(resp) => {
            let json: serde_json::Value = resp.json().unwrap_or_default();
            
            // MyMemory returns JSON with a "responseData" object containing "translatedText"
          //  json["responseData"]["translatedText"]
           json["responseData"]["translatedText"]
                .as_str()
                .unwrap_or("Translation Error")
                .to_string()
        }
        Err(_) => "Network Error".to_string(),
    }
}
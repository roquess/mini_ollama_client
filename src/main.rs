use std::io::{Read, Write};
use std::net::TcpStream;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting the Ollama client...");

    let server = "localhost:11434";
    let mut stream = TcpStream::connect(server)?;

    let prompt = "Hello ollama.";

    let request_body = format!(
        r#"{{
            "model": "phi3",
            "prompt": "{}",
            "options": {{
                "temperature": 0.2,
                "repeat_penalty": 1.5,
                "top_k": 25,
                "top_p": 0.25
            }}
        }}"#,
        prompt
    );

    let request = format!(
        "POST /api/generate HTTP/1.1\r\n\
         Host: localhost\r\n\
         Content-Type: application/json\r\n\
         Content-Length: {}\r\n\
         Connection: close\r\n\
         \r\n\
         {}",
        request_body.len(),
        request_body
    );

    stream.write_all(request.as_bytes())?;
    stream.flush()?;

    let mut response = Vec::new();
    stream.read_to_end(&mut response)?;

    let response_text = String::from_utf8_lossy(&response);
    let mut full_response = String::new();
    let mut in_body = false;

    for line in response_text.lines() {
        if line.is_empty() {
            in_body = true;
            continue;
        }

        if in_body {
            full_response.push_str(line);
            full_response.push('\n');
        }
    }

    let mut final_response = String::new();
    for line in full_response.lines() {
        if let Some(start) = line.find(r#""response":"#) {
            let response_start = start + r#""response":"#.len()+1;
            let response_part = &line[response_start..];
            if let Some(end) = response_part.find("\",\"") {
                final_response.push_str(&response_part[..end]);
            }
        }
    }

    if final_response.is_empty() {
        println!("No response received.");
    } else {
        println!("Response: {}", final_response.trim());
    }

    Ok(())
}


#[allow(unused_variables)]

use std::net::{TcpListener, TcpStream};
use std::io::{prelude::*, Error};
use std::fs;
use http_server::ThreadPool;
use regex::Regex;
use chrono;

mod body_templates{
    include!("../body_templates.rs");
}
use body_templates::HTTP_REQUEST_REGEX;


fn main(){
    let listener = TcpListener::bind("127.0.0.1:3030").unwrap();
    let pool: ThreadPool = ThreadPool::new(10);
    for stream in listener.incoming(){
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        })
    }
}


fn handle_connection(mut stream: TcpStream) {
    println!("Connection Established with server");
    let mut buffer = [0_u8; 1024];

    stream.read(&mut buffer).unwrap();

    let re = Regex::new(HTTP_REQUEST_REGEX).unwrap();

    let request_headers = String::from_utf8_lossy(&buffer[..]);
    let params = re.captures(&request_headers).unwrap();
    let method = params.get(1).map_or("", |v| v.as_str());
    let path = params.get(2).map_or("", |v| v.as_str());

    
    let current_date_time = chrono::offset::Utc::now().format("%a, %e %b %Y %T GMT").to_string();
    if method.to_lowercase() == "get"{
        let content : String = read_html_file(path).unwrap_or( "".to_string());
        let response = format!("HTTP/1.1 {} {}\r\nDate: {}\r\nServer: Fred/Server (Win32)\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}", 200, "OK", current_date_time, content.len(), content);
        stream.write(response.as_bytes()).unwrap();
    }
    else{
        println!("Wrong method");
    }
    // println!("Request {}", String::from_utf8_lossy(&buffer[..]));

    // let response = "Hello from RUST server".as_bytes();
   stream.flush().unwrap();
}

fn read_html_file(p: &str) -> Result<String, Error>{
    let path = format!("./html{}", if p == "/" { "/index.html" }else{ p });
    let file_data = fs::read_to_string(path)?;
    Ok(file_data)
}

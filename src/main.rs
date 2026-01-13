use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    
};
use std::fs::File;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let mut file = File::open("hello.html").unwrap();
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).unwrap();

    for stream in  listener.incoming()
    {
         let stream = stream.unwrap();
         println!("test");
         handle_connection(stream);
    }

    
}


fn   handle_connection (mut stream: TcpStream )
{
   let buf_reader = BufReader::new(&stream);
  //let http_request: Vec<_> = buf_reader
  //    .lines()
  //    .map(|result| result.unwrap())
  //    .take_while(|line| !line.is_empty())
  //    .collect();
     let   request_line =  buf_reader.lines().next().unwrap().unwrap();
     
    let  (status_line,filename) = if request_line == "GET / HTTP/1.1"   
       {
            ("HTTP/1.1 200 OK", "hello.html")
       }
       else 
       {
          ("HTTP/1.1 404 NOT FOUND", "404.html")
       };
     
             
             let contents = fs::read_to_string(filename).unwrap();
             let length = contents.len();

             let response =
                format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
             
            
             stream.write_all(response.as_bytes()).unwrap();

       }
       
    
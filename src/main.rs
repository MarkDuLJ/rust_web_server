use std::{fs, io::{BufRead, BufReader, Write}, net::{TcpListener, TcpStream}, thread, time::Duration};

use rust_web_server::ThreadPool;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8989").unwrap();

    let pool = ThreadPool::new(5);

    for stream in listener.incoming(){
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
    // for stream in listener.incoming(){
    //     let stream = stream.unwrap();
    //     thread::spawn(|| {
    //         handle_connection(stream);
    //     });
    // }

    
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    // let http_req: Vec<_> = buf_reader.lines()
    //         .map(|result| result.unwrap())
    //         .take_while(|line| !line.is_empty())
    //         .collect();

    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let (status_line, filename) = match &request_line[..]  {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" =>{
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
            _ =>("HTTP/1.1 404 NOT FOUND","404.html")
        };

    let content = fs::read_to_string(filename).unwrap();
    let len = content.len();
    
    let response = format!(
        "{status_line}\r\nContent_Length: {len}\r\n\r\n{content}"
    );
    stream.write_all(response.as_bytes()).unwrap();

}

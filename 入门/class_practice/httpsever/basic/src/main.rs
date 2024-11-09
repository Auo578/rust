use std::fs;
use std::thread;
use std::thread::sleep;
use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, BufReader, Write};
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    // stream拿到客户端和服务端的一个通道
    for stream in listener.incoming() { // stream 是 insult类型
        let mut stream = stream.unwrap(); // 因为在建立链接的时候有可能因为超时等失败
        // 拿到客户端和服务端的一个通道，之后读取数据
        // thread::spawn(||{
        //     hander_connection(stream);
        // });
        hander_connection(stream);
    }

}

fn hander_connection(mut stream:TcpStream ){
    let buf_reader = BufReader::new(&stream);


    let request_line = buf_reader.lines().next().unwrap().unwrap();
    //通过next选择第一行，第一行是请求行,因为返回的是option包裹的result
    println!("{}",request_line);

    //组装返回值

    let (status_line, filename) =  match request_line.as_ref() {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK","static/hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(std::time::Duration::from_secs(5));
            ("HTTP/1.1 200 OK","static/hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND","static/404.html"),
    };

    let content = fs::read_to_string(filename).unwrap();
    let length = content.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");
    stream.write_all(response.as_bytes()).unwrap();
    //会崩溃，因为还没做错误处理。
    //同时发起两个请求，串行，listner会在incoming阻塞，当第一个请求完成后才会进行第二个请求。要同时处理多个请求。
}

//1

    // if request_line == "GET / HTTP/1.1"{
    //     let status_line = "HTTP/1.1 200 OK\r\n";
    //     let content = fs::read_to_string("static/hello.html").unwrap();
    //     println!("{}",content);
    //     let length = content.len();
       
    //     let response = format!("{status_line}Content-Length: {length}\r\n\r\n{content}");
    //     stream.write_all(response.as_bytes()).unwrap();
    // }else if  request_line == "GET /sleep HTTP/1.1"{
    //     thread::sleep(std::time::Duration::from_secs(5));
    //     let status_line = "HTTP/1.1 200 OK\r\n";
    //     let content = fs::read_to_string("static/hello.html").unwrap();
    //     let length = content.len();
       
    //     let response = format!("{status_line}Content-Length: {length}\r\n\r\n{content}");
    //     stream.write_all(response.as_bytes()).unwrap();
    // }else {
    //     let status_line = "HTTP/1.1 404 NOT FOUND\r\n";
    //     let content = fs::read_to_string("static/404.html").unwrap();
    //     let length = content.len();
       
    //     let response = format!("{status_line}Content-Length: {length}\r\n\r\n{content}");
    //     stream.write_all(response.as_bytes()).unwrap();
    // }
   
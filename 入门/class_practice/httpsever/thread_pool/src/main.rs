use std::fs;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, BufReader, Write};
use std::thread::JoinHandle;

//根据API实现线程池

type job = Box<dyn FnOnce() + Send + 'static>;
struct Threadpool{
    works: Vec<thread::JoinHandle<()>>,
    sender: mpsc::Sender<job>,
}

impl Threadpool {
    fn new(size: usize) -> Threadpool{
        let mut works: Vec<thread::JoinHandle<()>> = Vec::with_capacity(size);

        let (sender, receiver) = mpsc::channel();
        //解决并发安全问题，加锁,arc多个线程共享
        let receiver = Arc::new(Mutex::new(receiver));


        for _ in 0..size{
            let receiver = Arc::clone(&receiver);

            let thread = thread::spawn(move || loop {
                //不立即执行,没有消息阻塞等待
                let job:job = receiver.lock().unwrap().recv().unwrap();
                job();
            });
            works.push(thread);
        }

        Threadpool{works,sender}
    }
    //因为不要求返回值就不设置返回值了

    //写入数据让线程执行，传递过来的消息是f
    fn execute<F>(&self, f: F)
    where 
        F: FnOnce(),
        F: Send + 'static,
    {
        self.sender.send(Box::new(f));
    }
}
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    // stream拿到客户端和服务端的一个通道

    let p = Threadpool::new(100);
    //设计对外API，放100个线程

    for stream in listener.incoming() { // stream 是 insult类型
        let mut stream = stream.unwrap(); // 因为在建立链接的时候有可能因为超时等失败
        // 拿到客户端和服务端的一个通道，之后读取数据

        //传入要执行的程序
        p.execute(||{
            hander_connection(stream);
        })//为线程池设计的API
        // thread::spawn(||{
        //     hander_connection(stream);
        // });
        // hander_connection(stream);
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
   
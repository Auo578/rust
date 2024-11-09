use std::collections;
use std::collections::HashMap;
use std::thread;

use module::request;
use module::request::Method;
use module::request::Value;
use tokio::fs;
use tokio::io::AsyncBufReadExt;
use tokio::io::AsyncWriteExt;
use tokio::io::BufReader;
use tokio::net::TcpListener;
use tokio::net::TcpStream;

use module::request::Request;
#[tokio::main]

//不能再使用标准库里的listner了，因为标准库里的listner是同步的
//在运行时里要使用tokio提供的listner
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    // 没有incoming，有accept，而且accept没有实现迭代器

    loop {
        if let Ok((stream, _)) = listener.accept().await {
            tokio::spawn(async{
                handle_connection(stream).await;
            });
            
        }   
    }

}


async fn handle_connection(mut stream:TcpStream ){
    //获取请求结构,使用Request的结构进行匹配
    let request = Request::from_stream(&mut stream).await;
    // request.path();
    // request.url();
    if request.is_ok(){
        let request = request.unwrap();
        println!("{:?}",request);
        // println!("{:?}",String::from_utf8(request.body.unwrap()));
        let query = request.url.query().unwrap();
        let query: Vec<(String,String)>= query.collect();
        println!("{:?}",query);
    
        let body = String::from_utf8(request.body.unwrap()).unwrap();
        let body = Value::parse(body.as_str());
        let body: HashMap<String,String>= body.collect();
        println!("{:?}",body);
    
    //request.url.path.as_str()
        let (status_line, filename) =  match request.method {
            Method::Get => ("HTTP/1.1 200 OK","static/hello.html"),
            _ => ("HTTP/1.1 405 NOT FOUND","static/404.html"),
            
            
            // "/" => ("HTTP/1.1 200 OK","static/hello.html"),
            // "/sleep " => {
            //     thread::sleep(std::time::Duration::from_secs(5));
            //     ("HTTP/1.1 200 OK","static/hello.html")
            // }
            // _ => ("HTTP/1.1 404 NOT FOUND","static/404.html"),
        };

        // let content = fs::read_to_string(filename).await.unwrap();
        // let length = content.len();
        let response = format!("HTTP/1.1 500 INTERVAL ERROR\r\n\r\n");
        stream.write_all(response.as_bytes()).await.unwrap();
    }




   
}

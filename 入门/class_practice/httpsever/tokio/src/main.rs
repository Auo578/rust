use std::thread;

use tokio::fs;
use tokio::io::AsyncBufReadExt;
use tokio::io::AsyncWriteExt;
use tokio::io::BufReader;
use tokio::net::TcpListener;
use tokio::net::TcpStream;

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
    // for stream in listener.accept() { // stream 是 insult类型
    //     let mut stream = stream.unwrap(); // 因为在建立链接的时候有可能因为超时等失败
    //     // 拿到客户端和服务端的一个通道，之后读取数据
    //     // thread::spawn(||{
    //     //     hander_connection(stream);
    //     // });
    //     hander_connection(stream);
    // }
}


async fn handle_connection(mut stream:TcpStream ){
    //对tokio来说需要使用可变引用stream
    let buf_reader = BufReader::new(&mut stream);

    //tokio，next是next.line(),使用await
    let request_line = buf_reader.lines().next_line().await.unwrap().unwrap();
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
    //await
    let content = fs::read_to_string(filename).await.unwrap();
    let length = content.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");
    stream.write_all(response.as_bytes()).await.unwrap();
    //会崩溃，因为还没做错误处理。
    //同时发起两个请求，串行，listner会在incoming阻塞，当第一个请求完成后才会进行第二个请求。要同时处理多个请求。
}

//学习里的示例
// use std::{sync::mpsc, thread};

// fn main() {
//     study();
// }

// fn study() {
//     let v = vec![1,2,3,4];
//     let handle = thread::spawn( move || {
//         for i in &v {
//             println!("{}", i);
//         }
//     });
//     handle.join().unwrap();

//     let val1 = mpsc::Sender::clone(&v);
// }

//题目一
// use std::fs::File;
// use std::io::{self, BufRead, BufReader};
// use std::sync::mpsc::{self, Receiver, Sender}; use std::sync::{Arc, Mutex};
// //其中self?
// use std::thread;
// use std::time::Duration;
// use crate::mpsc::SyncSender;

// // 定义处理文件的函数 process_file，读取文件内容并打印到控制台
// fn process_file(file_path: &str) -> io::Result<()> {
//     let file = File::open(file_path)?;
//     let reader = BufReader::new(file);

//     for line in reader.lines() {
//         println!("{}: {}", file_path, line?);
//     }

//     Ok(())
// }


// // 定义工作线程，负责从通道接收文件路径并处理文件
// fn worker(id: usize, rx: Arc<Mutex<Receiver<String>>>) {
//     loop {
//         // 锁定接收端，并从通道接收文件路径
//         let file_path = {
//             let rx = rx.lock().unwrap();  // 获取 Mutex 锁
//             rx.recv()  // 尝试接收消息
//         };

//         // 处理接收到的文件路径
//         match file_path {
//             Ok(file_path) => {
//                 println!("Worker {} started processing file: {}", id, file_path);
//                 if let Err(e) = process_file(&file_path) {
//                     eprintln!("Worker {} failed to process file {}: {}", id, file_path, e);
//                 }
//                 thread::sleep(Duration::from_secs(1));  // 模拟处理时间
//                 println!("Worker {} finished processing file: {}", id, file_path);
//             }
//             Err(_) => {
//                 println!("Worker {} is shutting down", id);
//                 break;  // 如果接收通道关闭，退出线程
//             }
//         }
//     }
// }

// fn main() {
//     // 文件路径列表，假设我们有10个文件要处理
//     let file_paths = vec![
//         "file1.txt", "file2.txt", "file3.txt", "file4.txt",
//         "file5.txt", "file6.txt", "file7.txt", "file8.txt",
//         "file9.txt", "file10.txt",
//     ];

//     // 创建一个带缓冲区的通道，最多允许4个文件同时处理
//     let (tx, rx): (SyncSender<String>, Receiver<String>) = mpsc::sync_channel(4);

//     // 将接收端放入 Arc 和 Mutex 中，以便多个线程共享
//     let rx = Arc::new(Mutex::new(rx));

//     // 创建4个工作线程
//     let num_threads = 4;
//     let mut handles = Vec::new();

//     for id in 0..num_threads {
//         let rx = Arc::clone(&rx);  // 通过 Arc 来共享接收端
//         let handle = thread::spawn(move || {
//             worker(id, rx);  // 每个线程运行 worker 函数
//         });
//         handles.push(handle);  // 将线程句柄保存起来，以便等待线程完成
//     }

//     // 主线程负责发送文件路径到通道中
//     for file_path in file_paths {
//         println!("Sending file {} to be processed.", file_path);
//         tx.send(file_path.to_string()).unwrap();  // 发送文件路径到通道
//     }

//     // 主线程关闭通道，通知所有工作线程文件发送完毕
//     drop(tx);

//     // 等待所有工作线程完成
//     for handle in handles {
//         handle.join().unwrap();  // 等待每个工作线程完成
//     }

//     println!("All files have been processed.");
// }



// //最后打出来的结果是有点混杂的，可以使用锁来确保一次只有一个线程可以打印输出


//2添加了停止信号里的优雅停止
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// 定义处理任务的函数
fn process_task(task_id: usize) {
    println!("Processing task: {}", task_id);
    thread::sleep(Duration::from_secs(1)); // 模拟任务处理时间
    println!("Finished task: {}", task_id);
}

// 定义工作线程，负责从通道接收任务并处理
fn worker(id: usize, task_rx: Arc<Mutex<Receiver<usize>>>, stop_rx: Arc<Mutex<Receiver<()>>>) {
    loop {
        // 尝试接收任务
        let task = {
            let task_rx = task_rx.lock().unwrap();
            task_rx.recv_timeout(Duration::from_millis(100)).ok()
        };

        // 检查停止信号
        let stop_signal = {
            let stop_rx = stop_rx.lock().unwrap();
            stop_rx.try_recv().ok()
        };

        if stop_signal.is_some() {
            println!("Worker {} received stop signal and is shutting down", id);
            break;
        }

        match task {
            Some(task_id) => {
                println!("Worker {} started processing task: {}", id, task_id);
                process_task(task_id);
                println!("Worker {} finished processing task: {}", id, task_id);
            }
            None => {
                // 如果任务通道已经关闭且没有任务，退出线程
                println!("Worker {} is shutting down", id);
                break;
            }
        }
    }
}

fn main() {
    // 创建任务通道和停止信号通道
    let (task_tx, task_rx) = mpsc::sync_channel(4);
    let (stop_tx, stop_rx) = mpsc::channel();

    let task_rx = Arc::new(Mutex::new(task_rx));
    let stop_rx = Arc::new(Mutex::new(stop_rx));

    // 创建工作线程
    let num_threads = 4;
    let mut handles = Vec::new();

    for id in 0..num_threads {
        let task_rx = Arc::clone(&task_rx);
        let stop_rx = Arc::clone(&stop_rx);
        let handle = thread::spawn(move || {
            worker(id, task_rx, stop_rx);
        });
        handles.push(handle);
    }

    // 主线程负责发送任务到通道中
    for task_id in 1..=10 {
        println!("Sending task {} to be processed.", task_id);
        task_tx.send(task_id).unwrap();
    }

    // 主线程关闭任务通道并发送停止信号
    drop(task_tx); // 关闭任务发送通道
    stop_tx.send(()).unwrap(); // 发送停止信号

    // 等待所有工作线程完成
    for handle in handles {
        handle.join().unwrap();
    }

    println!("All tasks have been processed.");
}

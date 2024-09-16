use std::{sync::mpsc, thread};

fn main() {
    study();
}

fn study() {
    let v = vec![1,2,3,4];
    let handle = thread::spawn( move || {
        for i in &v {
            println!("{}", i);
        }
    });
    handle.join().unwrap();

    let val1 = mpsc::Sender::clone(&v);
}

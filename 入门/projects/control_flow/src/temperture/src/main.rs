use std::to;

fn main(){
    println!("please enter c or f");

    let mut c = String::new();
    io::stdin()
        .read_line(&mut c)
        .expect("Fained to read line");
    let mut f = String::new();
}
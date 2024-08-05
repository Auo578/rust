
// fn main(){
//     let (x,y) = (1,2);
//     let s = sum(x,y);

//     assert_eq!(s,3);//assert_eq是宏，调用宏需要！号
// }

// fn sum(x:i32,y:i32) -> i32{
//     x+y
// }

// fn main(){
//     print();
// }

// fn print() {
//     println!("hello,world");
// }


// fn main() {
//     println!("Success!");
//     let option = get_option(1);
//     println!("{:?}", option);
// }

// fn get_option(tp: u8) -> Option<i32> {
//     match tp {
//         1 => {
//             Some(5)
//             // TODO
//         }
//         _ => {
//             never_return_fn()
//         }
//     }; 
//     // 这里与其返回一个 None，不如使用发散函数替代
//     never_return_fn()
// }

// // 使用三种方法实现以下发散函数
// fn never_return_fn() -> ! {
//     // panic!("This function never returns!");
//     //loop{}
//     unimplemented!()
//     // std::process::exit(1);   
// }


fn main() {
    // 填空
    let b = false; 

    let _v = match b {
        true => 1,
        // 发散函数也可以用于 `match` 表达式，用于替代任何类型的值
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic")
        }
    };

    println!("Exercise Failed if printing out this line!");
}
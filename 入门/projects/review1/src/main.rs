// // 猜数字游戏

// use std::io;
// use std::cmp::Ordering;
// use rand::Rng;
// fn main() {
//     println!("Guess the number!");
//     let secret_number = rand::thread_rng().gen_range(1..101);
//     // println!("The secret number is:{secret_number}");
   
//     loop {
//         println!("Please input your guess.");
//         let mut guess = String::new();
//         io::stdin().read_line(&mut guess).expect("Failed to read line");
//         let guess: u32 = match guess.trim().parse() {
//             Ok(num) => num,
//             Err(_) => continue,
//         };
//         println!("You guessed: {}", guess);
//         match guess.cmp(&secret_number) {
//             Ordering::Equal => {
//                 println!("You win!");
//                 break;
//             }
//             Ordering::Greater => println!("Too big!"),
//             Ordering::Less => println!("Too small!"),
//         }
//     }

// }



// // 修复错误
// fn main() {
//     let x = define_x();
//     println!("{}, world", x); 
// }

// fn define_x() -> String{
//     let x = "hello".to_string();
//     x
// }

// // 修复错误
// fn main() {
//     let x = define_x();
//     println!("{:?}, world", x); 
// }

// fn define_x() -> &'static str {
//     let x = "hello";
//     x
// }


// fn main() {
//     let x = 1; 
// }

// // compiler warning: unused variable: `x`


// 修复下面代码的错误并尽可能少的修改
// fn main() {
//     let ( x, y) = (1, 2);
//     let x = 3; // 重新绑定

//     assert_eq!(x, 3);
//     assert_eq!(y, 2);
// }


// fn main() {
//     let (x, y);
//     (x,..) = (3, 4);
//     [.., y] = [1, 2];
//     // 填空，让代码工作
//     assert_eq!([x,y], [3, 2]);
// } 


// // 修改 `assert_eq!` 让代码工作
// fn main() {
//     let x= 5;
//     assert_eq!("i32".to_string(), type_of(&x));
// }

// // 以下函数可以获取传入参数的类型，并返回类型的字符串形式，例如  "i8", "u8", "i32", "u32"
// fn type_of<T>(_: &T) -> String {
//     format!("{}", std::any::type_name::<T>())
// }


// 将 ? 替换成你的答案
// fn main() {
//     let x = 1_000.000_1; // ?
//     let y: f32 = 0.12; // f32
//     let z = 0.01_f64; // f64
// }

// fn main() {
//     let mut sum = 0;
//     for i in -3..2 {
//         sum += i
//     }

//     assert!(sum == -5);

//     for c in 'a'..='z' {
//         println!("{}",c);
//     }
// }


// // 填空，并解决错误
// fn main() {
//     // 整数加法
//     assert!(1u32 + 2 == 3);

//     // 整数减法
//     assert!(1i32 - 2 == -1);
//     assert!(1i8 - 2 == -1);
    
//     assert!(3 * 50 == 150);

//     assert!((9.6f64 / 3.2 - 3.0f64).abs()<= 0.00001); // error ! 修改它让代码工作

//     assert!(24 % 5 == 4);
    
//     // 逻辑与或非操作
//     assert!(true && false == false);
//     assert!(true || false == true);
//     assert!(!true == false);

//     // 位操作
//     println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
//     println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
//     println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
//     println!("1 << 5 is {}", 1u32 << 5);
//     println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
// }

// 修改一行让代码正常打印
// fn main() {
//     let c1 = '中';
//     print_char(c1);
// } 

// fn print_char(c : char) {
//     println!("{}", c);
// }


// 让代码工作，但不要修改 `implicitly_ret_unit` !
// fn main() {
//     let v1: () = ();

//     let v = (2, 3);
//     assert_eq!(v1, implicitly_ret_unit());

//     println!("Success!")
// }

// fn implicitly_ret_unit() {
//     println!("I will return a ()")
// }

// // 不要使用下面的函数，它只用于演示！
// fn explicitly_ret_unit() -> () {
//     println!("I will return a ()")
// }


// 让代码工作：修改 `assert!` 中的 `4` 
// use std::mem::size_of_val;
// fn main() {
//     let unit: () = ();
//     assert!(size_of_val(&unit) == 0);

//     println!("Success!")
// }

// fn main() {
//     let x = 5u32;

//     let y = {
//         let x_squared = x * x;
//         let x_cube = x_squared * x;

//         // 下面表达式的值将被赋给 `y`
//         x_cube + x_squared + x
//     };

//     let z = {
//         // 分号让表达式变成了语句，因此返回的不再是表达式 `2 * x` 的值，而是语句的值 `()`
//         2 * x;
//     };

//     println!("x is {:?}", x);
//     println!("y is {:?}", y);
//     println!("z is {:?}", z);
// }

// 使用两种方法让代码工作起来
// fn main() {
//     let v = {
//         let mut x = 1;
//         x + 2
//     };
 
//     assert_eq!(v, 3);
//  }

// // 使用两种方法让代码工作起来
// fn main() {
//     let v = {
//         let mut x = 1;
//         x += 2
//     };
 
//     assert_eq!(v, ());
//  }




// fn main() {
//     let v = x;
//     let x = 3;
 
//     assert!(v == 3);
//  }


// fn main() {
//     let s = sum(1 , 2);
//     assert_eq!(s, 3);
// }

// fn sum(x: i32, y: i32) -> i32 {
//     return x + y;
// }

// fn main() {
//     let s = sum(1 , 2);
//     assert_eq!(s, 3);
// }

// fn sum(x: i32, y: i32) -> i32 {
//     x + y
// }


// fn main() {
//     let s = sum(1 , 2);
//     assert_eq!(s, 3);
// }

// fn sum(x: i32, y: i32) -> i32 {
//     x + y
// }


// fn main() {
//     // 不要修改下面两行代码!
//     let (x, y) = (1, 2);
//     let s = sum(x, y);

//     assert_eq!(s, 3);
// }

// fn sum(x: i32, y: i32) -> i32{
//     x + y
// }

// fn main() {
//     print();
//  }
 
//  // 使用另一个类型来替代 i32
//  fn print() -> () {
//     println!("hello,world");
//  }



// 用两种方法求解
// fn main() {
//     never_return();
// }

// fn never_return() -> ! {
//     panic!();
    
// }

// fn main() {
//     println!("Success!");
// }

// fn get_option(tp: u8) -> Option<i32> {
//     match tp {
//         1 => {
//             // TODO
//         }
//         _ => {
//             // TODO
//         }
//     };
    
//     // 这里与其返回一个 None，不如使用发散函数替代
//     never_return_fn()
// }

// // 使用三种方法实现以下发散函数
// fn never_return_fn() -> ! {
//     panic!();
// }


// fn main() {
//     // 填空
//     let b = false;

//     let _v = match b {
//         true => 1,
//         // 发散函数也可以用于 `match` 表达式，用于替代任何类型的值
//         false => {
//             println!("Success!");
//             panic!("we have no value for `false`, but we can panic")
//         }
//     };

//     println!("Exercise Failed if printing out this line!");
// }


// fn main() {
//     // 使用尽可能多的方法来通过编译
//     let x = String::from("hello, world");
//     let y = &x;
//     println!("{},{}",x,y);
// }

// 不要修改 main 中的代码
// fn main() {
//     let s1 = String::from("hello, world");
//     let s2 = take_ownership(s1);

//     println!("{}", s2);
// }

// // 只能修改下面的代码!
// fn take_ownership(s: String) -> String {
//     s
// }


// fn main() {
//     let s = give_ownership();
//     println!("{}", s);
// }

// // 只能修改下面的代码!
// fn give_ownership() -> String {
//     let s = String::from("hello, world");
//     // convert String to Vec
//     // 将 String 转换成 Vec 类型
//     // let _s = s.into_bytes();
//     s
// }

// 修复错误，不要删除任何代码行
// fn main() {
//     let s = String::from("hello, world");

//     print_str(&s);

//     println!("{}", s);
// }

// fn print_str(s: &str)   {
//     println!("{}",s);
// }

// 不要使用 clone，使用 copy 的方式替代
// fn main() {
//     let x = (1, 2, (), "hello");
//     let y = x;
//     println!("{:?}, {:?}", x, y);
// }


// fn main() {
//     let s = String::from("hello, ");
    
//     // 只修改下面这行代码 !
//     let mut s1 = s;

//     s1.push_str("world")
// }


// fn main() {
//     let x = Box::new(5);
    
//     let mut y=x.clone();      // 完成该行代码，不要修改其它行！
    
//     *y = 4;
    
//     assert_eq!(*x, 5);
// }


// fn main() {
//     let t = (String::from("hello"), String::from("world"));
 
//     let _s = t.0;
 
//     // 仅修改下面这行代码，且不要使用 `_s`
//     println!("{:?}", t.1);
//  }


// fn main() {
//     let t = (String::from("hello"), String::from("world"));
 
//     // 填空，不要修改其它代码
//     let (s1, s2) = (&t.0, &t.1);
 
//     println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
//  }


// fn main() {
//     let x = 5;
//     // 填写空白处
//     let p = &x;
 
//     println!("x 的内存地址是 {:p}", p); // output: 0x16fa3ac84
//  }


// fn main() {
//     let x = 5;
//     let y = &x;

//     // 只能修改以下行
//     assert_eq!(5, *y);
// }


// 修复错误
// fn main() {
//     let mut s = String::from("hello, ");

//     borrow_object(&s)
// }

// fn borrow_object(s: &String) {}


// 修复错误
// fn main() {
//     let mut s = String::from("hello, ");

//     push_str(&mut s);
// }

// fn push_str(s: &mut String)   {
//     s.push_str("world")
// }


// fn main() {
//     let mut s = String::from("hello, ");

//     // 填写空白处，让代码工作
//     let p = &mut s;
    
//     p.push_str("world");
// }


// fn main() {
//     let c = '中';

//     let r1 = &c;
//     // 填写空白处，但是不要修改其它行的代码
//     let ref r2 = c;

//     assert_eq!(*r1, *r2);
    
//     // 判断两个内存地址的字符串是否相等
//     assert_eq!(get_addr(r1),get_addr(r2));
// }

// // 获取传入引用的内存地址的字符串形式
// fn get_addr(r: &char) -> String {
//     format!("{:p}", r)
// }


// 移除代码某个部分，让它工作
// 你不能移除整行的代码！
// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &s;
//     let r2 = &s;

//     println!("{}, {}", r1, r2);
// }


// fn main() {
//     // 通过修改下面一行代码来修复错误
//     let  mut s = String::from("hello, ");

//     borrow_object(&mut s)
// }

// fn borrow_object(s: &mut String) {}


// 注释掉一行代码让它工作
// fn main() {
//     let mut s = String::from("hello, ");

//     let r1 = &mut s;
//     r1.push_str("world");
//     let r2 = &mut s;
//     r2.push_str("!");
    
//     // println!("{}",r1);
// }


// fn main() {
//     let mut s = String::from("hello, ");

//     let r1 = &mut s;
//     let r2 = &mut s;
//     println!("{}",r1);

//     // 在下面增加一行代码人为制造编译错误：cannot borrow `s` as mutable more than once at a time
//     // 你不能同时使用 r1 和 r2
// }


// // 使用至少两种方法来修复错误
// fn main() {
//     let s: Box<str> = "hello, world".into();
//     greetings(&s)
// }

// fn greetings(s: &str) {
//     println!("{}",s)
// }


// // 使用至少两种方法来修复错误
// fn main() {
//     let s: &str = "hello, world";
//     greetings(s)
// }

// fn greetings(s: &str) {
//     println!("{}",s)
// }


// 修复所有错误，并且不要新增代码行
// fn main() {
//     let  mut s = String::from("hello");
//     s.push(',');
//     s.push_str(" world");
//     s.push('!');

//     println!("{}", s)
// }


// // 修复所有错误，不要删除任何一行代码
// fn main() {
//     let s1 = String::from("hello,");
//     let s2 = "world!";
//     let s3 = s1.clone() + s2; 
//     assert_eq!(s3,"hello,world!");
//     println!("{}",s1);
// }


// 使用至少两种方法来修复错误
// fn main() {
//     let s = "hello, world";
//     greetings(s)
// }

// fn greetings(s: &str) {
//     println!("{}",s)
// }


// 使用至少两种方法来修复错误
// fn main() {
//     let s = String::from("hello, world");
//     greetings(s)
// }

// fn greetings(s: String) {
//     println!("{}",s)
// }

/* 填空并修复所有错误 */

// fn main() {
//     // 原始字符串不会解析转义字符，因此将原始字符串改为直接包含正确的字符
//     let raw_str = r#"Escapes don't work here: ? ℝ"#;
//     assert_eq!(raw_str, "Escapes don't work here: ? ℝ");

//     // 如果你希望在字符串中使用双引号，可以直接包含它们，而不需要转义
//     let quotes = r#"And then I said: "There is no escape!""#;
//     println!("{}", quotes);

//     // 如果希望在字符串中使用 # 号，可以如下使用
//     let delimiter = r###"A string with "# in it. And even "##!"###;
//     println!("{}", delimiter);

//     // 填空：在原始字符串中，可以直接写双引号而不需要使用反斜杠
//     let long_delimiter = r###"Hello, "##""###;
//     assert_eq!(long_delimiter, "Hello, \"##\"");
// }


// fn main() {
//     let s1 = String::from("hi,中国");
//     let h = &s1[0..1]; // 修改当前行来修复错误，提示: `h` 字符在 UTF-8 格式中只需要 1 个字节来表示
//     assert_eq!(h, "h");

//     let h1 = &s1[3..6];// 修改当前行来修复错误，提示: `中` 字符在 UTF-8 格式中需要 3 个字节来表示
//     assert_eq!(h1, "中");
// }


// fn main() {
//     // 填空，打印出 "你好，世界" 中的每一个字符
//     for c in "你好，世界".chars() {
//         println!("{}", c)
//     }
// }

// use utf8_slice;
// fn main() {
//     let s = "The 🚀 goes to the 🌑!";

//     let rocket = utf8_slice::slice(s, 4, 5);
//     // 结果是 "🚀"
//     println!("{rocket}")
// }

// fn main() {
//     let names = [String::from("Sunfei"), "Sunface".to_string()];
    
//     // `get` 返回 `Option<T>` 类型，因此它的使用非常安全
//     let name0 = names.get(0).unwrap();

//     // 但是下标索引就存在越界的风险了
//     let name1 = &names[1];

//     dbg!(name0,name1);
// }


// 修复代码中的错误，不要新增代码行!
// fn main() {
//     let arr = [1, 2, 3];
//     let s1: &[i32] = &arr[0..2];

//     let s2: &str = "hello, world" ;
// }


// 修复代码错误
// fn main() {
//     let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
//     println!("too long tuple: {:?}", too_long_tuple);
// }


// struct Unit;
// trait SomeTrait {
//     // ...定义一些行为
// }

// // 我们并不关心结构体中有什么数据( 字段 )，但我们关心它的行为。
// // 因此这里我们使用没有任何字段的单元结构体，然后为它实现一些行为
// impl SomeTrait for Unit {  }
// fn main() {
//     let u = Unit;
//     do_something_with_unit(u);
// } 

// // 填空，让代码工作
// fn do_something_with_unit(u: Unit) {   }


// 填空
// struct Person {
//     name: String,
//     age: u8,
// }
// fn main() {} 

// fn build_person(name: String, age: u8) -> Person {
//     Person {
//         age,
//         name,
//     }
// }


// 修复错误
// enum Number {
//     Zero,
//     One,
//     Two,
// }

// enum Number1 {
//     Zero = 0,
//     One,
//     Two=2,
// }

// // C语言风格的枚举定义
// enum Number2 {
//     Zero = 0.0,
//     One = 1.0,
//     Two = 2.0,
// }


// fn main() {
//     // 通过 `as` 可以将枚举值强转为整数类型
//     assert_eq!(Number::One, Number1::One.as u8);
//     assert_eq!(Number1::One, Number2::One.as u32);
// } 


// // 填空让 `println` 输出，同时添加一些代码不要让最后一行的 `panic` 执行到
// fn main() {
//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);

//     if let Some(n) = six {
//         println!("{}", n);
//         return
//     } 
        
//     panic!("不要让这行代码运行！");
// } 

// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i + 1),
//     }
// }


// 填空，让代码运行
// use crate::List::*;

// #[derive(Debug)]
// enum List {
//     // Cons: 链表中包含有值的节点，节点是元组类型，第一个元素是节点的值，第二个元素是指向下一个节点的指针
//     Cons(u32, Box<List>),
//     // Nil: 链表中的最后一个节点，用于说明链表的结束
//     Nil,
// }

// // 为枚举实现一些方法
// impl List {
//     // 创建空的链表
//     fn new() -> List {
//         // 因为没有节点，所以直接返回 Nil 节点
//         // 枚举成员 Nil 的类型是 List
//         Nil
//     }

//     // 在老的链表前面新增一个节点，并返回新的链表
//     fn prepend(self, elem: u32) -> List {
//         Cons(elem, Box::new(self))
//     }

//     // 返回链表的长度
//     fn len(&self) -> u32 {
//         match *self {
//             // 这里我们不能拿走 tail 的所有权，因此需要获取它的引用
//             Cons(u32, ref tail) => 1 + tail.len(),
//             // 空链表的长度为 0
//             Nil => 0
//         }
//     }

//     // 返回链表的字符串表现形式，用于打印输出
//     fn stringify(&self) -> String {
//         match *self {
//             Cons(head, ref tail) => {
//                 // 递归生成字符串
//                 format!("{}, {}", head, tail.stringify())
//             },
//             Nil => {
//                 format!("Nil")
//             },
//         }
//     }
// }

// fn main() {
//     // 创建一个新的链表(也是空的)
//     let mut list = List::new();

//     // 添加一些元素
//     list = list.prepend(1);
//     list = list.prepend(2);
//     list = list.prepend(3);

//     // 打印列表的当前状态
//     println!("链表的长度是: {}", list.len());
//     println!("{}", list.stringify());

//     println!("{:?}",list);
// }


// 修复错误
// fn main() {
//     let n = 5;

//     let big_n =
//         if n < 10 || n > -10 {
//             println!(" 数字太小，先增加 10 倍再说");

//             10 * n
//         } else {
//             println!("数字太大，我们得让它减半");

//             n / 2 
//         };

//     println!("{} -> {}", n, big_n);
// } 


// fn main() {
//     for n in 1..100 { // 修改此行，让代码工作
//         if n == 100 {
//             panic!("NEVER LET THIS RUN")
//         }
//     }
// } 


// 修复错误，不要新增或删除代码行
// fn main() {
//     let names = [String::from("liming"),String::from("hanmeimei")];
//     for name in &names {
//         // do something with name...
//     }

//     println!("{:?}", names);

//     let numbers = [1, 2, 3];
//     // numbers中的元素实现了 Copy，因此无需转移所有权
//     for n in numbers {
//         // do something with name...
//     }
    
//     println!("{:?}", numbers);
// // } 
// fn main() {
//     let a = [4,3,2,1];

//     // 通过索引和值的方式迭代数组 `a` 
//     for (i,v) in a.iter().enumerate() {
//         println!("第{}个元素是{}",i+1,v);
//     }
//     dbg!(a);
// }


// 填空，让最后一行的  println! 工作 !
// fn main() {
//     // 一个计数值
//     let mut n = 1;

//     // 当条件为真时，不停的循环
//     while n < 10 {
//         if n % 15 == 0 {
//             println!("fizzbuzz");
//         } else if n % 3 == 0 {
//             println!("fizz");
//         } else if n % 5 == 0 {
//             println!("buzz");
//         } else {
//             println!("{}", n);
//         }
        
//         n += 5;
//     }

//     println!("n 的值是 {}, 循环结束",n);
// }


// 填空
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// fn main() {
//     let msgs = [
//         Message::Quit,
//         Message::Move{x:1, y:3},
//         Message::ChangeColor(255,255,0)
//     ];

//     for msg in msgs {
//         show_message(msg)
//     }
// } 

// fn show_message(msg: Message) {
//     match msg {
//         Message::Move { x:a, y:b } => { // 这里匹配 Message::Move
//             assert_eq!(a, 1);
//             assert_eq!(b, 3);
//         },
//         Message::ChangeColor(_, g, b) => {
//             assert_eq!(g, 255);
//             assert_eq!(b, 0);
//         }
//         _ => println!("no data in these variants")
//     }
// }


// fn main() {
//     let alphabets = ['a', 'E', 'Z', '0', 'x', '9' , 'Y'];

//     // 使用 `matches` 填空
//     for ab in alphabets {
//         assert!(matches!(ab, 'a'..='z' | '0'..='9' | 'A'..='Z'));
//     }
// } 


// enum MyEnum {
//     Foo,
//     Bar
// }

// fn main() {
//     let mut count = 0;

//     let v = vec![MyEnum::Foo,MyEnum::Bar,MyEnum::Foo];
//     for e in v {
//         if matches!(e, MyEnum::Foo) { // 修复错误，只能修改本行代码
//             count += 1;
//         }
//     }

//     assert_eq!(count, 2);
// }


// fn main() {
//     let o = Some(7);

//     if let Some(i) = o {
//         println!("This is a really long string and `{:?}`", i);
//     }
//     // 移除整个 `match` 语句块，使用 `if let` 替代
//     // match o {
//     //     Some(i) => {
//     //         println!("This is a really long string and `{:?}`", i);
//     //     }
//     //     _ => {}
//     // };
// }


// 填空
// enum Foo {
//     Bar(u8)
// }

// fn main() {
//     let a = Foo::Bar(1);

//     if let Foo::Bar(i) = a {
        
//         println!("foobar 持有的值是: {}", i);
//     }
// }


// enum Foo {
//     Bar,
//     Baz,
//     Qux(u32)
// }

// fn main() {
//     let a = Foo::Qux(10);

//     match a {
//         Foo::Bar => println!("match foo::bar"),
//         Foo::Baz => println!("match foo::baz"),
//         _ => println!("match others")
//     }

//     // 移除以下代码，使用 `match` 代替
//     // if let Foo::Bar = a {
//     //     println!("match foo::bar")
//     // } else if let Foo::Baz = a {
//     //     println!("match foo::baz")
//     // } else {
//     //     println!("match others")
//     // }
// }


// 就地修复错误
// fn main() {
//     let age = Some(30);
//     if let Some(age) = age { // 创建一个新的变量，该变量与之前的 `age` 变量同名
//        assert_eq!(age, 30);
//     } // 新的 `age` 变量在这里超出作用域
    
//     match age {
//         // `match` 也能实现变量遮蔽
//         Some(age) =>  println!("age 是一个新的变量，它的值是 {}",age),
//         _ => ()
//     }
//  }


// 修复错误
// enum Message {
//     Hello { id: i32 },
// }

// fn main() {
//     let msg = Message::Hello { id: 10 };

//     match msg {
//         Message::Hello {
//             id: id@ 3..=7,
//         } => println!("id 值的范围在 [3, 7] 之间: {}", id),
//         Message::Hello { id: newid@ 10..=12 } => {
//             println!("id 值的范围在 [10, 12] 之间: {}", newid)
//         }
//         Message::Hello { id } => println!("Found some other id: {}", id),
//     }
// }

// use std::arch::x86_64;


// // 填空让代码工作，必须使用 `split`
// fn main() {
//     let num = Some(4);
//     let split = 5;
//     match num {
//         Some(x) if x < split => assert!(x < split),
//         Some(x)  => assert!(x >= split),
//         None => (),
//     }
// }


// 修复错误，尽量少地修改代码
// 不要移除任何代码行
// fn main() {
//     let mut v = String::from("hello,");
//     let r = &mut v;

//     match r {
//        value => {value.push_str(" world!");dbg!(value);}
//     };
// }

// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     // 完成 area 方法，返回矩形 Rectangle 的面积
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

// fn main() {
//     let rect1 = Rectangle { width: 30, height: 50 };

//     assert_eq!(rect1.area(), 1500);
// }

// 只填空，不要删除任何代码行!
// #[derive(Debug)]
// struct TrafficLight {
//     color: String,
// }

// impl TrafficLight {
//     pub fn show_state(&self)  {
//         println!("the current state is {}", self.color);
//     }
// }
// fn main() {
//     let light = TrafficLight{
//         color: "red".to_owned(),
//     };
//     // 不要拿走 `light` 的所有权
//     light.show_state();
//     // 否则下面代码会报错
//     println!("{:?}", light);
// }

// struct TrafficLight {
//     color: String,
// }

// impl TrafficLight {
//     // 使用 `Self` 填空
//     pub fn show_state(&self)  {
//         println!("the current state is {}", self.color);
//     }

//     // 填空，不要使用 `Self` 或其变体
//     pub fn change_state(&mut self) {
//         self.color = "green".to_string()
//     }
// }
// fn main() {}

// #[derive(Debug)]
// struct TrafficLight {
//     color: String,
// }

// impl TrafficLight {
//     // 1. 实现下面的关联函数 `new`,
//     // 2. 该函数返回一个 TrafficLight 实例，包含 `color` "red"
//     // 3. 该函数必须使用 `Self` 作为类型，不能在签名或者函数体中使用 `TrafficLight`
//     pub fn new() -> Self {
//         Self {
//             color: "red".to_string(),
//         }
//     }

//     pub fn get_state(&self) -> &str {
//         &self.color
//     }
// }

// fn main() {
//     let light = TrafficLight::new();
//     assert_eq!(light.get_state(), "red");
// }


// #[derive(Debug)]
// enum TrafficLightColor {
//     Red,
//     Yellow,
//     Green,
// }

// // 为 TrafficLightColor 实现所需的方法
// impl TrafficLightColor {
//     fn color(&self) -> &str {
//         match self {
//             TrafficLightColor::Red => "red",
//             TrafficLightColor::Yellow => "yellow",
//             TrafficLightColor::Green => "green",
//         }
//     }
// }
// fn main() {
//     let c = TrafficLightColor::Yellow;

//     assert_eq!(c.color(), "yellow");

//     println!("{:?}",c);
// }
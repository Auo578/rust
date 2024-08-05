// use serde::{Serialize, Deserialize};
// use serde_json;

// fn main() {

//         // 创建 Origin 结构体的一个实例
//         let origin = Origin {
//             name: String::from("John Doe"),
//             age: 30,
//             email: String::from("john.doe@example.com"),
//             address: Address{
//                 street: Some(String::from("123 Main St")),
//                 city: Some(String::from("AA")),
//             }
//         };

//     let json_origin = serde_json::to_string(&origin).unwrap();
//     println!("{}",json_origin);

//     let msg = serde_json::to_string(&origin);
//     match  msg {
//         Ok(json_origin) => println!("{}",json_origin),
//         Err(e) => eprintln!("Error to json: {}",e),
//     }
// }


// #[derive(Serialize, Deserialize, Debug)]
// struct Origin{
//     name : String,
//     age : usize,
//     email : String,
//     address: Address,
// }

// #[derive(Serialize, Deserialize, Debug)]
// struct  Address{
//     street:Option<String>,
//     city:Option<String>,
// }

//match practice
// // 1填空
// enum Direction {
//     East,
//     West,
//     North,
//     South,
// }

// fn main() {
//     let dire = Direction::South;
//     match dire {
//         Direction::East => println!("East"),
//         Direction::North  => { // 在这里匹配 South 或 North
//             println!("South or North");
//         },
//         _ => println!("West"),
//     };
// }

//2
// fn main() {
//     let boolean = true;

//     // 使用 match 表达式填空，并满足以下条件
//     //
//     // boolean = true => binary = 1
//     // boolean = false => binary = 0
//     let binary = match boolean  {
//         true =>  1,
//         false =>   0,
//     };

//     assert_eq!(binary, 1);
// }



// 3填空
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
//         Message::Move{x:a,y:b} => { // 这里匹配 Message::Move
//             assert_eq!(a, 1);
//             assert_eq!(b, 3);
//         },
//         Message::ChangeColor(_, g, b) => {
//             assert_eq!(g, 255);
//             assert_eq!(b, 0);
//         }
//         __ => println!("no data in these variants")
//     }
// }

//4

// fn main() {
//     let alphabets = ['a', 'E', 'Z', '0', 'x', '9' , 'Y'];

//     // 使用 `matches` 填空
//     for ab in alphabets {
//         assert!(matches!(ab,alphabets))   
// assert!(matches!(ab, 'a'..='z' | 'A'..='Z' | '0'..='9'))
//     }
// } 

//5

// enum MyEnum {
//     Foo,
//     Bar
// }

// fn main() {
//     let mut count = 0;

//     let v = vec![MyEnum::Foo,MyEnum::Bar,MyEnum::Foo];
//     for e in v {
//         if let MyEnum::Foo = e { // 修复错误，只能修改本行代码  //if matches!(e, MyEnum::Foo )
//             count += 1;
//         }
//     }

//     assert_eq!(count, 2);
// }

// //6
// fn main() {
//     let o = Some(7);

//     // 移除整个 `match` 语句块，使用 `if let` 替代
//     if let Some(i) = o{
//         println!("This is a really long string and `{:?}`", i);
//     }
  
// }


// 7填空
// enum Foo {
//     Bar(u8)
// }

// fn main() {
//     let a = Foo::Bar(1);

//     if let Foo::Bar(i) = a   {
//         println!("foobar 持有的值是: {}", i);
//     }
// }

//8

// enum Foo {
//     Bar,
//     Baz,
//     Qux(u32)
// }

// fn main() {
//     let a = Foo::Qux(10);

//     // 移除以下代码，使用 `match` 代替
//     match a {
//         Foo::Bar =>  println!("match foo::bar"),
//         Foo::Baz =>  println!("match foo::baz"),
//         _ =>  println!("match others"),
//     }
//     // if let Foo::Bar = a {
//     //     println!("match foo::bar")
//     // } else if let Foo::Baz = a {
//     //     println!("match foo::baz")
//     // } else {
//     //     println!("match others")
//     // }
// }



// 9就地修复错误
// fn main() {
//     let age = Some(30);
//     if let Some(a) = age { // 创建一个新的变量，该变量与之前的 `age` 变量同名
//        assert_eq!(age, Some(30));
//     } // 新的 `age` 变量在这里超出作用域
    
//     match age {
//         // `match` 也能实现变量遮蔽
//         Some(age) =>  println!("age 是一个新的变量，它的值是 {}",age),
//         _ => ()
//     }
//  }


//model

//1
// fn main() {}
// fn match_number(n: i32) {
//     match n {
//         // 匹配一个单独的值
//         1 => println!("One!"),
//         // 使用 `|` 填空，不要使用 `..` 或 `..=`
//         2|3|4|5 => println!("match 2 -> 5"),
//         // 匹配一个闭区间的数值序列
//         6..=10 => {
//             println!("match 6 -> 10")
//         },
//         _ => {
//             println!("match 11 -> +infinite")
//         }
//     }
// }


//2

// struct Point {
//     x: i32,
//     y: i32,
// }

// fn main() {
//     // 填空，让 p 匹配第二个分支
//     let p = Point { x: 5, y: 10 };

//     match p {
//         Point { x, y: 0 } => println!("On the x axis at {}", x),
//         // 第二个分支
//         Point { x: 0..=5, y: y@ (10 | 20 | 30) } => println!("On the y axis at {}", y),
//         Point { x, y } => println!("On neither axis: ({}, {})", x, y),
//     }
// }


// 3修复错误
// enum Message {
//     Hello { id: i32 },
// }

// fn main() {
//     let msg = Message::Hello { id: 5 };

//     match msg {
//         Message::Hello {
//             id:  id @3..=7,
//         } => println!("id 值的范围在 [3, 7] 之间: {}", id),
//         Message::Hello { id: newid@10 ..= 12 } => {
//             println!("id 值的范围在 [10, 12] 之间: {}", newid)
//         }
//         Message::Hello { id} => println!("Found some other id: {}", id),
//     }
// }


// 4填空让代码工作，必须使用 `split`
// fn main() {
//     let num = Some(4);
//     let split = 5;
//     match num {
//         Some(x) if x< split => assert!(x < split),   //
//         Some(x) if x>= split => assert!(x >= split),  //Some(x) => assert!(x >= split),
//         Some(_)=>(),
//         None => (),
//     }
// }



// 5填空，让代码工作
// fn main() {
//     let numbers = (2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048);

//     match numbers {
//         (first,..,last) => {
//            assert_eq!(first, 2);
//            assert_eq!(last, 2048);
//         }
//     }
// }


// 6修复错误，尽量少地修改代码
// 不要移除任何代码行
// fn main() {
//     let mut v = String::from("hello,");
//     let r = &mut v;

//     match r {
//         value => value.push_str(" world!") 
//     }
// }

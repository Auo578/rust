//课程第一题
// struct Stack<T>{
//     element:Vec<T>,
// }
// impl <T> Stack<T> {
//     fn peek(&self) -> Option<&T>{
//         self.element.last()
//     }    

//     fn push(&mut self, item:T) {
//         self.element.push(item)
//     }

//     fn pop(&mut self) -> Option<T>{
//         self.element.pop()
//     }
// }




// fn main() {
//     let mut s = Stack{element:vec![1,2,3,4]};
//     s.push(5);
//     println!("{:?}",s.element);
//     println!("{:?}",s.pop());
//     println!("{:?}",s.peek());
// }

//课程第二题
// use std::collections::HashMap;

// fn main(){
//     let text = "ahd asjhdkj  asdkhkas dasjk ahd";
//     let mut  map = HashMap::new();

//     for word in text.split_whitespace(){
//         let count = map.entry(word).or_insert(0);
//         *count += 1;
//     }

//     println!("{map:?}");
// }

// use core::time;
//课程第三题
// use std::{collections::HashMap, io::{self, Write}};
// struct  Inventory{
//     books: HashMap<String,u32>,
// }

// impl Inventory{
//     //新的库存管理系统
//     fn new() -> Inventory{
//         Inventory{
//             books: HashMap::new(),
//         }
//     }
//     //添加书
//     fn add_book(&mut self, title: &str, stock: u32){
//         self.books.insert(title.to_string(), stock);
//     }

//     //查询库存
//     fn get_stock(&self, title: &str) -> Option<u32> {
//         self.books.get(title).copied()
//     }

//     //更新库存
//     fn update_stock(&mut self, title: &str, stock: u32){
//         if self.books.contains_key(title){
//             self.books.insert(title.to_string(), stock);
//         }else {
//             println!("Book '{}' dose not exist in inventory.", title);
//         }
//     }
//     fn delete_book(&mut self, title: &str){
//         if self.books.remove(title).is_none(){
//             println!("book '{}' not found in inventory. ",title);
//         }
//     }

//     fn list_books(&self){
//         for (title,stock) in &self.books {
//             println!("title:{},stock:{}.",title,stock);
//         }
//     }
// }


// fn main(){
//     let mut inventory = Inventory::new();

//     loop{
//         println!("书籍库存管理系统");
//         println!("1. 添加书籍");
//         println!("2. 查询库存");
//         println!("3. 更新库存");
//         println!("4. 删除书籍");
//         println!("5. 列出所有书籍");
//         println!("6. 退出");
//         print!("请输入操作编号: ");
//         io::stdout().flush().unwrap();

//         let mut input = String::new();
//         io::stdin().read_line(&mut input).unwrap();
//         let choice: u32 = input.trim().parse().unwrap_or(0);

//         match choice {
//             1 => {
//                 let mut title = String::new();
//                 let mut  stock = String::new();
//                 println!("请输入书名：");
//                 io::stdout().flush().unwrap();
//                 io::stdin().read_line(&mut title).unwrap();
//                 println!("请输入库存数量: ");
//                 io::stdout().flush().unwrap();
//                 io::stdin().read_line(&mut stock).unwrap();
//                 let stock: u32 = stock.trim().parse().unwrap_or(0);
//                 inventory.add_book(title.trim(), stock);
//                 println!("书籍已添加。");
//             }

//             2 => {
//                 let mut title = String::new();
//                 println!("请输入要查询的书名：");
//                 io::stdin().read_line(&mut title).unwrap();
//                 match inventory.get_stock(title.trim()){
//                     Some(stock) => println!("title:{},stock:{}.",  title.trim(),stock),
//                     None => println!("书'{}'不在库存内。", title.trim())
//                 }
//             }
            
//             3 => {
//                 let mut title = String::new();
//                 let mut stock = String::new();

//                 println!("请输入要更新的书名: ");
//                 io::stdout().flush().unwrap();
//                 io::stdin().read_line(&mut title).unwrap();
//                 println!("请输入新的库存数量: ");
//                 io::stdout().flush().unwrap();
//                 io::stdin().read_line(&mut stock).unwrap();

//                 let stock = stock.trim().parse().unwrap_or(0);
//                 inventory.update_stock(title.trim(), stock);

//             }

//             4 => {
//                 let mut title = String::new();
//                 println!("请输入要删除的书籍：");
//                 io::stdout().flush().unwrap();
//                 io::stdin().read_line(&mut title).unwrap();
//                 inventory.delete_book(title.trim());
//             }

//             5 => {
//                 inventory.list_books();
//             }

//             6 =>{
//                 println!("退出");
//                 break;
//             }

//             _ =>{
//                 println!("输入无效，重新输入");
//             }
//         }
//     }
// }


// Rust by practice collection String

//1

// 填空并修复错误
// 1. 不要使用 `to_string()`
// 2. 不要添加/删除任何代码行
// fn main() {
//     let mut s: String = String::from("hello, ");
//     s.push_str("world!");
//     s.push_str("__");

//     move_ownership(&s);

//     assert_eq!(s, "hello, world!__");

//     println!("Success!")
// }

// fn move_ownership(s: &String) {
//     println!("ownership of \"{}\" is moved here!", &s)
// }

//2
// 填空
// fn main() {  
//     let mut s = String::from("hello, world");
 
//     let slice1: &str = &s; // 使用两种方法
//     assert_eq!(slice1, "hello, world");
 
//     let slice2 = String::from("hello");
//     assert_eq!(slice2, "hello");
 
//     let slice3: &mut String = &mut s; 
//     slice3.push('!');
//     assert_eq!(slice3, "hello, world!");
 
//     println!("Success!")
//  }

//  fn main() {  
//     let mut s = String::from("hello, world");
 
//     let slice1: &str = s.as_str(); // 使用两种方法
//     assert_eq!(slice1, "hello, world");
 
//     let slice2 = &s[0..5];
//     assert_eq!(slice2, "hello");
 
//     // let slice3: __ = __; 
//     // slice3.push('!');
//     // assert_eq!(slice3, "hello, world!");
 
//     println!("Success!")
//  }


//3

// // 问题:  我们的代码中发生了多少次堆内存分配？
// // 你的回答: 2
// fn main() {  
//     // 基于 `&str` 类型创建一个 String,
//     // 字符串字面量的类型是 `&str`
//    let s: String = String::from("hello, world!");

//    // 创建一个切片引用指向 String `s`
//    let slice: &str = &s;

//    // 基于刚创建的切片来创建一个 String
//    let s: String = slice.to_string();

//    assert_eq!(s, "hello, world!");

//    println!("Success!")
// }

//4

// 填空并修复错误
// fn main() {
//     let s = String::from("hello, 世界");
//     let slice1 = &s[0..1]; //提示: `h` 在 UTF-8 编码中只占用 1 个字节
//     assert_eq!(slice1, "h");

//     let slice2 = &s[7..10];// 提示: `世` 在 UTF-8 编码中占用 3 个字节
//     assert_eq!(slice2, "世");
    
//     // 迭代 s 中的所有字符
//     for (i, c) in s.chars().enumerate() {
//         if i == 7 {
//             assert_eq!(c, '世')
//         }
//     }

//     println!("Success!")
// }

//5

// // 填空
// fn main() {
//     let mut s = String::new();
//     s.push_str("hello");

//     let v = vec![104, 101, 108, 108, 111];

//     // 将字节数组转换成 String
//     let s1 = String::from_utf8(v).unwrap();
    
    
//     assert_eq!(s, s1);

//     println!("Success!")
// }

//6
// fn main() {
//     let mut s = String::with_capacity(25);

//     println!("{}", s.capacity());

//     for _ in 0..2 {
//         s.push_str("hello");
//         println!("{}", s.capacity());
//     }

//     println!("Success!")
// }


// 填空
// use std::mem;

// fn main() {
//     let story = String::from("Rust By Practice");

//     // 阻止 String 的数据被自动 drop
//     let mut story = mem::ManuallyDrop::new(story);

//     let ptr = story.as_mut_ptr();
//     let len = story.len();
//     let capacity = story.capacity();

//     assert_eq!(16, len);

//     // 我们可以基于 ptr 指针、长度和容量来重新构建 String. 
//     // 这种操作必须标记为 unsafe，因为我们需要自己来确保这里的操作是安全的
//     let s = unsafe { String::from_raw_parts(ptr, len, capacity) };

//     assert_eq!(*story, s);

//     println!("Success!")
// }


//vec
//1--解是解了，但很不好，多看看答案怎么做的


// fn main() {
//     let arr: [u8; 3] = [1, 2, 3];
    
//     let v = Vec::from(arr);
//     is_vec(v.clone());

//     let v = vec![1, 2, 3];
//     is_vec(v.clone());

//     // vec!(..) 和 vec![..] 是同样的宏，宏可以使用 []、()、{}三种形式，因此...
//     let v = vec!(1, 2, 3);
//     is_vec(v.clone());
    
//     // ...在下面的代码中, v 是 Vec<[u8; 3]> , 而不是 Vec<u8>
//     // 使用 Vec::new 和 `for` 来重写下面这段代码
//     let mut v1 = Vec::new();
//     is_vec(v1.clone());
//     for i in 1..4{
//         v1.push(i);
//     }
 
//     assert_eq!(v, v1);

//     println!("Success!")
// }

// fn is_vec(v: Vec<u8>) {}

//2

// 填空
// fn main() {
//     let mut v1 = Vec::from([1, 2, 4]);
//     v1.pop();
//     v1.push(3);
    
//     let mut v2 = Vec::new();
//     v2.extend([1,2,3]);

//     assert_eq!(v1, v2);

//     println!("Success!")
// }

//3

// 填空
// fn main() {
//     // array -> Vec
//     // impl From<[T; N]> for Vec
//     let arr = [1, 2, 3];
//     let v1 = vec!(1,2,3);
//     let v2: Vec<i32> = arr.to_vec();
//     // println!("{:?},{:?}",v1,v2);
//     assert_eq!(v1, v2);
 
    
//     // String -> Vec
//     // impl From<String> for Vec
//     let s = "hello".to_string();
//     let v1: Vec<u8> = s.try_into().unwrap();

//     let s = "hello".to_string();
//     let v2 = s.into_bytes();
//     assert_eq!(v1, v2);

//     // impl<'_> From<&'_ str> for Vec
//     let s = "hello";
//     let v3 = Vec::from(s);
//     assert_eq!(v2, v3);

//     // 迭代器 Iterators 可以通过 collect 变成 Vec
//     let v4: Vec<i32> = [0; 10].into_iter().collect();
//     println!("{:?}",v4);
//     assert_eq!(v4, vec![0; 10]);

//     println!("Success!")
//  }


// 4修复错误并实现缺失的代码
// fn main() {
//     let mut v = Vec::from([1, 2, 3]);
//     for i in 0..5 {
//         println!("{:?}", v.get(i))
//     }

//     for i in 0..5 {
//        if let Some(x) = v.get(i){
//         v[i] = x + 1;
//        }else {
//            v.push(i+2);
//        }
//     }
    
//     assert_eq!(format!("{:?}",v), format!("{:?}", vec![2, 3, 4, 5, 6]));

//     println!("Success!")
// }

//5

// use std::intrinsics::mir::Len;

// 修复错误
// fn main() {
//     let mut v = vec![1, 2, 3];

//     let slice1 = &v[..];
//     // 越界访问将导致 panic.
//     // 修改时必须使用 `v.len`
//     let slice2 = &v[0..3];
    
//     assert_eq!(slice1, slice2);
    
//     // 切片是只读的
//     // 注意：切片和 `&Vec` 是不同的类型，后者仅仅是 `Vec` 的引用，并可以通过解引用直接获取 `Vec`
//     let vec_ref: &mut Vec<i32> = &mut v;
//     (*vec_ref).push(4);
//     let slice3 = &mut v[0..4];
//     // slice3[3]=4;

//     assert_eq!(slice3, &[1, 2, 3, 4]);

//     println!("Success!")
// }

//6
// 修复错误
// fn main() {
//     let mut vec = Vec::with_capacity(10);

//     assert_eq!(vec.len(), 0);
//     assert_eq!(vec.capacity(), 10);

//     // 由于提前设置了足够的容量，这里的循环不会造成任何内存分配...
//     for i in 0..10 {
//         vec.push(i);
//     }
//     assert_eq!(vec.len(), 10);
//     assert_eq!(vec.capacity(), 10);

//     // ...但是下面的代码会造成新的内存分配
//     vec.push(11);
//     assert_eq!(vec.len(), 11);
//     assert!(vec.capacity() >= 11);


//     // 填写一个合适的值，在 `for` 循环运行的过程中，不会造成任何内存分配
//     let mut vec = Vec::with_capacity(100);
//     for i in 0..100 {
//         vec.push(i);
//     }

//     assert_eq!(vec.len(), 100);
//     assert_eq!(vec.capacity(), 100);
    
//     println!("Success!")
// }

//7
// #[derive(Debug,PartialEq)]
// enum IpAddr {
//     V4(String),
//     V6(String),
// }
// fn main() {
//     // 填空
//     let v : Vec<IpAddr>= vec! [
//         IpAddr::V4("127.0.0.1".to_string()),
//         IpAddr::V6("::1".to_string())
//     ];
    
//     // 枚举的比较需要派生 PartialEq 特征
//     assert_eq!(v[0], IpAddr::V4("127.0.0.1".to_string()));
//     assert_eq!(v[1], IpAddr::V6("::1".to_string()));

//     println!("Success!")
// }

//8
// trait IpAddr {
//     fn display(&self);
// }

// struct V4(String);
// impl IpAddr for V4 {
//     fn display(&self) {
//         println!("ipv4: {:?}",self.0)
//     }
// }
// struct V6(String);
// impl IpAddr for V6 {
//     fn display(&self) {
//         println!("ipv6: {:?}",self.0)
//     }
// }

// fn main() {
//     // 填空
//     let v:Vec<Box<dyn IpAddr>> = vec![
//         Box::new(V4("127.0.0.1".to_string())),
//         Box::new(V6("::1".to_string())),
//     ];

//     for ip in v {
//         ip.display();
//     }
// }

//hashmap
//1

// 填空并修复错误
// 

//2

// use std::collections::HashMap;
// fn main() {
//     let teams = [
//         ("Chinese Team", 100),
//         ("American Team", 10),
//         ("France Team", 50),
//     ];

//     let mut teams_map1 = HashMap::new();
//     for team in &teams {
//         teams_map1.insert(team.0, team.1);
//     }
//     // println!("{:?}",teams_map1);

//     // // 使用两种方法实现 team_map2
//     // // 提示:其中一种方法是使用 `collect` 方法
//     // let teams_map2 = teams_map1.clone();
//        let teams_map2 = HashMap::from(teams);
//     let teams_map2 = teams.iter().cloned().collect();
//     assert_eq!(teams_map1, teams_map2);

//     println!("Success!")
// }

//3

// 填空
// use std::collections::HashMap;
// fn main() {
//     // 编译器可以根据后续的使用情况帮我自动推断出 HashMap 的类型，当然你也可以显式地标注类型：HashMap<&str, u8>
//     let mut player_stats = HashMap::new();

//     // 查询指定的 key, 若不存在时，则插入新的 kv 值
//     player_stats.entry("health").or_insert(100);

//     assert_eq!(player_stats["health"], 100);

//     // 通过函数来返回新的值
//     player_stats.entry("health").or_insert_with(random_stat_buff);
//     assert_eq!(player_stats["health"], 100);

//     let health = player_stats.entry("health").or_insert(50);
//     assert_eq!(health, &100);
//     *health -= 50;
//     assert_eq!(*health, 50);

//     println!("Success!")
// }

// fn random_stat_buff() -> u8 {
//     // 为了简单，我们没有使用随机，而是返回一个固定的值
//     42
// }

//4

// 修复错误
// 提示: `derive` 是实现一些常用特征的好办法
// use std::collections::HashMap;
// #[derive(Debug, Hash, Eq, PartialEq)]
// struct Viking {
//     name: String,
//     country: String,
// }

// impl Viking {
//     fn new(name: &str, country: &str) -> Viking {
//         Viking {
//             name: name.to_string(),
//             country: country.to_string(),
//         }
//     }
// }

// fn main() {
//     // 使用 HashMap 来存储 viking 的生命值
//     let vikings = HashMap::from([
//         (Viking::new("Einar", "Norway"), 25),
//         (Viking::new("Olaf", "Denmark"), 24),
//         (Viking::new("Harald", "Iceland"), 12),
//     ]);

//     // 使用 derive 的方式来打印 viking 的当前状态
//     for (viking, health) in &vikings {
//         println!("{:?} has {} hp", viking, health);
//     }
// }

// 修复错误，尽可能少的去修改代码
// 不要移除任何代码行！
// use std::collections::HashMap;
// fn main() {
//   let v1 = 10;
//   let mut m1 = HashMap::new();
//   m1.insert(v1, v1);
//   println!("v1 is still usable after inserting to hashmap : {}", v1);

//   let v2 = "hello".to_string();
//   let mut m2 = HashMap::new();
//   // 所有权在这里发生了转移
//   m2.insert(&v2, v1);

//   assert_eq!(v2, "hello");

//    println!("Success!")
// }

use std::collections::HashMap;

fn main() {
    let v1 = 10;
    let mut m1 = HashMap::new();
    m1.insert(v1, v1);
    println!("v1 is still usable after inserting to hashmap : {}", v1);

    // &str implements Copy trait
    let v2 = "hello";
    let mut m2 = HashMap::new();
    m2.insert(v2, v1);

    assert_eq!(v2, "hello");
}
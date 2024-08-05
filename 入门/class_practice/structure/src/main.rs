//课程作业，实现debug
// use std::fmt;

// impl fmt::Debug for Rectangle {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_struct("Rectangle")
//             .field("width", &self.width)
//             .field("height", &self.height)
//             .finish()
//     }
// }

// struct Rectangle{
//     width:u32,
//     height:u32,
// }
// fn main(){
//     let rect1 = Rectangle{
//         width:10,
//         height:10,
//     };

//     println!("rect1 is {:?}",rect1);
// }


//practice


// 1fix the error
// struct Person {
//     name: String,
//     age: u8,
//     hobby: String
// }
// fn main() {
//     let age = 30;
//     let p = Person {
//         name: String::from("sunface"),
//         age,
//         hobby: String::from("fix")
//     };
// } 

//2
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
// fn do_something_with_unit(u: Unit) {   } //impl SomeTrait


// 3填空并修复错误
// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);
// fn main() {
//     let v = Point(0, 127, 255);
//     check_color(v);
// }   

// fn check_color(p: Point) {
//     let Point(x, _, _) = p;
//     assert_eq!(x, 0);
//     assert_eq!(p.1, 127);
//     assert_eq!(p.2, 255);
//  }


// 4填空并修复错误，不要增加或移除代码行
// struct Person {
//     name: String,
//     age: u8,
// }
// fn main() {
//     let age = 18;
//     let mut p = Person {
//         name: String::from("sunface"),
//         age,
//     };

//     // how can you believe sunface is only 18? 
//     p.age = 30;

//     // 填空
//     p.name = String::from("sunfei");
// }



// 5填空
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


// 6填空，让代码工作
// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }
// fn main() {
//     let u1 = User {
//         email: String::from("someone@example.com"),
//         username: String::from("sunface"),
//         active: true,
//         sign_in_count: 1,
//     };

//     let u2 = set_email(u1);
// } 

// fn set_email(u: User) -> User {
//     User {
//         email: String::from("contact@im.dev"),
//         ..u
//     }
// }


// 7填空，让代码工作
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let scale = 2;
//     let rect1 = Rectangle {
//         width: dbg!(30 * scale), // 打印 debug 信息到标准错误输出 stderr,并将 `30 * scale` 的值赋给 `width`
//         height: 50,
//     };

//     dbg!(&rect1); // 打印 debug 信息到标准错误输出 stderr

//     println!("{:?}", rect1); // 打印 debug 信息到标准输出 stdout
// }


// 8修复错误
// #[derive(Debug)]
// struct File {
//     name: String,
//     data: String,
// }
// fn main() {
//     let f = File {
//         name: String::from("readme.md"),
//         data: "Rust By Practice".to_string()
//     };

//     let _name = f.name;

//     // 只能修改这一行
//     println!("{}, {}",&_name, &f.data);
// } 
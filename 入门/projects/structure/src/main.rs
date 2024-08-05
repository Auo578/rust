// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn main() {
//     let user1 = User{
//         active:true,
//         username: String::from("someusername123"),
//         email: String::from("someone@example.com"),
//         sign_in_count:1,
//     };

//     user1.email = String::from("anotheremail@example.com");
// }

// fn build_user(wmail:String, username:String) -> User{
//     User{
//         active:true,
//         username,
//         email,
//         sign_in_count:1,
//     }
// }

// fn main(){
//     let user2 = User{
//         active:user1.active,
//         username:user1.username,
//         email:String::from("another@example.com"),
//         sign_in_count:user1.sign_in_count,
//     };
// }

// fn main(){
//     let user2=User{
//         email:String::from("another@example.com"),
//         ..user1
//     };
// }

// fn main(){
//     #[derive(Debug)]
//     struct Rectangle{
//         width:u32,
//         height:u32,
//     }

//     impl Rectangle {
//         fn area (&self) -> u32{
//             self.width * self.height
//         }
//     };

//     let rect1 = Rectangle{
//         width:30,
//         height:50,
//     };

//     println!(
//         "The area of rectangle is {} square pixels.",rect1.area()
//     );
// }

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 关联函数：创建一个正方形
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let sq = Rectangle::square(3);
    println!("The width and height of the square are {}.", sq.width);
}

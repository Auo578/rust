// #[derive(Debug)]
// enum UsState{
//     Alabama,
//     Alaska,
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }
// //coin是参数名，Coin是参数

// fn value_in_cents(coin:Coin) -> u8{
//     match coin{
//         Coin::Penny => {
//             println!("Lucky penny");
//             1
//         }
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State quarter from {state:?}!");
//             25
//         }
//     }
// }
// fn main() {
//     let value = value_in_cents(Coin::Quarter(UsState::Alaska));
//     println!("{}", value);
// }

//use std::collections::linked_list;

// fn plus_one(x: Option<i32>) -> Option<i32>{
//     match x {
//         None => None,
//         Some(i) => Some(i+1),
//     }
// }

// fn main(){
//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);
//     println!("{:?},{:?},{:?}",five,six,none);
// }

// fn main() {
//     let dice_roll = 9;
//     match dice_roll {
//         3 => add_fancy_hat(),
//         7 => remove_fancy_hat(),
//         _ => (),
//     }
    
// }
// fn add_fancy_hat(){}
// fn remove_fancy_hat(){}

// let config_max = Some(3u8);
// match config_max{
//     some(max) => println!("The maximum is configured to be {max}"),
//     _ => (),
// }

// let config_max = Some(3u8);
// if let some(max) => config_max{
//     println!("The maximum is configured to be {max}"),
// }

// let mut count = 0;
// if let Coin::Quarter(state) = coin{
//     println!("State quarter from {state:?}!");
// }else{
//     count += 1;
// }
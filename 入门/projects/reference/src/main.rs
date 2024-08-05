
// fn main(){
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1);

//     println!("The length of '{}' is {}.",s1, len);
// }

// fn calculate_length(s:&String) -> usize{
//     s.len()
// }

// fn main(){
//     let mut s = String::from("hello");

//     change(&mut s);
//     println!("{s}");
// }

// fn change(some_string: &mut String){
//     some_string.push_str(",world");
// }

// fn main(){
//     let mut s = String::from("hello");

//     let r1 = &mut s;
//     let r2 = &mut s;

//     println!("{}, {}",r1,r2);
// // }
// fn main(){
//     let mut s = String::from("hello");
//     let r1 = &s;
//     let r2 = &s;
//     println!("{},{}",r1,r2);

//     let r3 =&mut s;
//     println!("{}",r3);
// }
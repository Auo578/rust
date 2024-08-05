// fn main() {
//     let s = string::from("hello");

//     takes_ownership(s);

//     let x = 5;

//     makes_copy(x);
// }

// fn takes_ownership(some_string:string){
//     println!("{}",some_string);
// }

// fn makes_copy(some_integer: i32){
//     println!("{}",some_integer);
// }
// fn main(){
//     let s1 = gives_ownership();

//     let s2 = String::from("hello");

//     let s3 = takes_and_gives_back(s2);

//     println!("{s1},{s3}");
// }

// fn gives_ownership() -> String{
//     let some_string = String::from("yours");

//     some_string
// }

// fn takes_and_gives_back(a_string: String) -> String{
//     a_string
// }

// fn main(){
//     let s1 = String::from("hello");

//     let(s2, len) = calculate_length(s1);

//     println!("The length of '{}' is {}.",s2, len);
// }

// fn calculate_length(s:String) -> (String, usize){
//     let length = s.len();

//     (s,length)
// }
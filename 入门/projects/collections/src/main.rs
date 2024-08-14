// fn main() {
//     let v: Vec<i32> = Vec::new();
//     let mut v1 =vec![1,2,3];
//     v1.push(4);

//     let third: &i32 = &v1[2];
//     println!("The third element is {third}");

//     let third:Option<&i32> = v1.get(2);
//     match third {
//         Some(third) => println!("The third element is {third}"),
//         None => println!("There is no third element"),
//     }

//     let v3 = vec![100,32,57];
//     for i in &v3{
//         println!("{i}");
//     }

//     let mut v4: Vec<i32> = vec![100,32,57];
//     for i in &mut v4{
//         *i += 50;
//         println!("{i}");
//     }

//     enum SpreadsheetCell {
//         Int(i32),
//         Float(f64),
//         Text(String),
//     }

//     let row = vec![
//         SpreadsheetCell::Int(3),
//         SpreadsheetCell::Text(String::from("blue")),
//         SpreadsheetCell::Float(10.12),
//     ];

// }



fn main(){
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

}
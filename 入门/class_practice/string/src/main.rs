
// 1修复错误，不要新增代码行
// fn main() {
//     let s:&str = "hello, world";
// }


// 2使用至少两种方法来修复错误
//2.1
// fn main() {
//     let s: Box<str> = "hello, world".into();
//     greetings(&s)
// }

// fn greetings(s: &str) {
//     println!("{}",s)
// }

//2.2

// 使用至少两种方法来修复错误
// fn main() {
//     let s = "hello, world".into();
//     greetings(s)
// }

// fn greetings(s: &str) {
//     println!("{}",s)
// }

// 2.3
// fn main() {
//     let s: Box<str> = "hello, world".into();
//     greetings(s)
// }

// fn greetings(s: Box<str>) {
//     println!("{}",s)
// }

//2.4

// 使用至少两种方法来修复错误
// fn main() {
//     let s: Box<&str> = "hello, world".into();
//     greetings(*s)
// }

// fn greetings(s: &str) {
//     println!("{}",s)
// }

//3

// 填空
// fn main() {
//     let mut s = String::from("");
//     s.push_str("hello, world");
//     s.push('!');

//     assert_eq!(s, "hello, world!");
// }


// 4 修复所有错误，并且不要新增代码行
// fn main() {
//     let  mut s = String::from("hello");
//     s.push(',');
//     s.push_str(" world");
//     s += "!";

//     println!("{}", s)
// }


// 5 填空
// fn main() {
//     let s = String::from("I like dogs");
//     // 以下方法会重新分配一块内存空间，然后将修改后的字符串存在这里
//     let s1 = s.replace("dogs", "cats");

//     assert_eq!(s1, "I like cats")
// }

//6

// 修复所有错误，不要删除任何一行代码
// fn main() {
//     let s1 = String::from("hello,");
//     let s2 = String::from("world!");
//     let s3 = s1.clone() + &s2; 
//     assert_eq!(s3,"hello,world!");
//     println!("{}",s1);
// }

//7.1

// // 使用至少两种方法来修复错误
// fn main() {
//     let s = "hello, world";
//     greetings(s)
// }

// fn greetings(s: &str) {
//     println!("{}",s)
// }


// 7.2使用至少两种方法来修复错误
// fn main() {
//     let s = String::from("hello, world");
//     greetings(s)
// }

// fn greetings(s: String) {
//     println!("{}",s)
// }


// 8.1使用两种方法来解决错误，不要新增代码行
// fn main() {
//     let s = "hello, world";
//     let s1: &str = s;
// }

// 8..2使用两种方法来解决错误，不要新增代码行
// fn main() {
//     let s = "hello, world".to_string();
//     let _s1: String = s;
// }
//9
// fn main() {
//     // 你可以使用转义的方式来输出想要的字符，这里我们使用十六进制的值，例如 \x73 会被转义成小写字母 's'
//     // 填空以输出 "I'm writing Rust"
//     let byte_escape = "I'm writing Ru\x73\x74!";
//     println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

//     // 也可以使用 Unicode 形式的转义字符
//     let unicode_codepoint = "\u{211D}";
//     let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

//     println!("Unicode character {} (U+211D) is called {}",
//                 unicode_codepoint, character_name );

//     // 还能使用 \ 来连接多行字符串
//     let long_string = "String literals\
//                         can span multiple lines.\
//                         The linebreak and indentation here \
//                          can be escaped too!";
//     println!("{}", long_string);
// }

//10
/* 填空并修复所有错误 */
// fn main() {
//     let raw_str = "Escapes don't work here: \x3F \u{211D}";
//     // 修改上面的行让代码工作
//     assert_eq!(raw_str, "Escapes don't work here: ? ℝ");

//     // 如果你希望在字符串中使用双引号，可以使用以下形式
//     let quotes = r#"And then I said: "There is no escape!""#;
//     println!("{}", quotes);

//     // 如果希望在字符串中使用 # 号，可以如下使用：
//     let  delimiter = r###"A string with "# in it. And even "##!"###;
//     println!("{}", delimiter);

//     // 填空
//     let long_delimiter = r###"Hello, "##""###;
//     assert_eq!(long_delimiter, "Hello, \"##\"")
// }

//11

// fn main() {
//     let s1 = String::from("hi,中国");
//     let h = &s1[0..1]; // 修改当前行来修复错误，提示: `h` 字符在 UTF-8 格式中只需要 1 个字节来表示
//     assert_eq!(h, "h");

//     let h1 = &s1[3..6];// 修改当前行来修复错误，提示: `中` 字符在 UTF-8 格式中需要 3 个字节来表示
//     assert_eq!(h1, "中");
// }

//12

// fn main() {
//     // 填空，打印出 "你好，世界" 中的每一个字符
//     for c in "你好，世界".chars() {
//         println!("{}", c)
//     }
// }
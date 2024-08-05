
// fn first_word(s:&String) -> usize{
//     let bytes = s.as_bytes();
//     for(i, &item) in bytes.iter().enumerate(){
//         if item == b' '{
//             return i;
//         }
//     }

//     s.len()
//     //为在 s.len() 之后有一个分号 ;，这会使函数隐式地返回 () 类型而不是 usize。
// }

// fn main() {
//     let mut s = String::from("hello world");
//     let word = first_word(&s);
//     s.clear();
//     println!("{word}");
// }


fn first_word(s:&str) -> &str{
    let bytes = s.as_bytes();
    for(i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i]
        }
    }

    &s[..]

}


// fn main() {
//     let mut s = String::from("hello world");
//     {let word = first_word(&s);
//     println!("{word}");
//     }
//     s.clear();
// }
// //因为 clear 需要清空 String，它尝试获取一个可变引用。
// //在调用 clear 之后的 println! 使用了 word 中的引用，所以这个不可变的引用在此时必须仍然有效。
// //Rust 不允许 clear 中的可变引用和 word 中的不可变引用同时存在，因此编译失败。

fn main() {
    let my_string = String::from("hello world");

    // `first_word` 适用于 `String`（的 slice），部分或全部
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` 也适用于 `String` 的引用，
    // 这等价于整个 `String` 的 slice
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` 适用于字符串字面值，部分或全部
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // 因为字符串字面值已经 **是** 字符串 slice 了，
    // 这也是适用的，无需 slice 语法！
    let word = first_word(my_string_literal);
}




//课程题目

fn main(){
    let nums = vec![2,3,4,5];
    let result = product(nums);
    println!("{:?}",result);
}
fn product(nums:Vec<i32>) -> Vec<i32>{
    let n = nums.len();
    let mut answer = vec![1;n];

    for i in 1..n{
        answer[i] = answer[i-1] * nums[i-1];
    } 
    let mut right = 1;
    for i in (1..n).rev(){
        answer[i] *= right;
        right = nums[i];
    }

    answer
}

//practice 1

// fn main() {
//     // 使用合适的类型填空
//     let arr:[i32;5] = [1, 2, 3, 4, 5];

//     // 修改以下代码，让它顺利运行
//     assert!(arr.len() == 5);
// }


//2
// fn main() {
//     // 很多时候，我们可以忽略数组的部分类型，也可以忽略全部类型，让编译器帮助我们推导
//     let _arr0 = [1, 2, 3];
//     let arr: [char; 3] = ['a', 'b', 'c'];
    
//     // 填空
//     // 数组分配在栈上， `std::mem::size_of_val` 函数会返回整个数组占用的内存空间
//     // 数组中的每个 char 元素占用 4 字节的内存空间，因为在 Rust 中， char 是 Unicode 字符
//     assert!(std::mem::size_of_val(&arr) == 12);
// }

//3

// fn main() {
//     // 填空
//     let list: [i32; 100] = [1;100];

//     assert!(list[0] == 1);
//     assert!(list.len() == 100);
// }

//4

// fn main() {
//     // 修复错误
//     let _arr = [1, 2, 3];
// }

//5

// fn main() {
//     let arr = ['a', 'b', 'c'];
    
//     let ele = arr[0]; // 只修改此行来让代码工作

//     assert!(ele == 'a');
// }


// 6 修复代码中的错误
// fn main() {
//     let names = [String::from("Sunfei"), "Sunface".to_string()];
    
//     // `get` 返回 `Option<T>` 类型，因此它的使用非常安全
//     let _name0 = names.get(0).unwrap();

//     // 但是下标索引就存在越界的风险了
//     let _name1 = &names[1];
// }


//1

// 修复代码中的错误，不要新增代码行!
// fn main() {
//     let arr = [1, 2, 3];
//     let _s1: &[i32] = &arr[0..2];

//     let _s2: &str = "hello, world" ;
// }

//2
// fn main() {
//     let arr: [char; 3] = ['中', '国', '人'];

//     let slice = &arr[..2];
    
//     // 修改数字 `8` 让代码工作
//     // 小提示: 切片和数组不一样，它是引用。如果是数组的话，那下面的 `assert!` 将会通过： '中'和'国'是char类型，char类型是Unicode编码，大小固定为4字节，两个字符为8字节。
//     assert!(std::mem::size_of_val(&slice) == 16);
// }

//3

// fn main() {
//     let arr: [i32; 5] = [1, 2, 3, 4, 5];
//    // 填空让代码工作起来
//    let slice: &[i32] = &arr[1..4];
//    assert_eq!(slice, &[2, 3, 4]);
//  }

//4

// fn main() {
//     let s = String::from("hello");

//     let slice1 = &s[0..2];
//     // 填空，不要再使用 0..2
//     let slice2 = &s[..2];

//     assert_eq!(slice1, slice2);
// }

// 5
// fn main() {
//     let s = "你好，世界";
//     // 修改以下代码行，让代码工作起来
//     let slice = &s[0..3];

//     assert!(slice == "你");
// }

//6

// // 修复所有错误
// fn main() {
//     let mut s = String::from("hello world");

//     // 这里, &s 是 `&String` 类型，但是 `first_character` 函数需要的是 `&str` 类型。
//     // 尽管两个类型不一样，但是代码仍然可以工作，原因是 `&String` 会被隐式地转换成 `&str` 类型，如果大家想要知道更多，可以看看 Deref 章节: https://course.rs/advance/smart-pointer/deref.html
//     let ch = first_character(&s);

    

//     println!("the first character is: {}", ch);
//     s.clear(); // error!
// }
// fn first_character(s: &str) -> &str {
//     &s[..1]
// }
fn main() {
    test_lifetime_multiple();
}

//解法一：删除Vec！里的'a 
// fn test_lifetime_multiple(){
//     fn insert_value<'a,'b>(my_vec:&'a mut Vec<i32>, value:&'b i32){
//         my_vec.push(*value);  //这里需要解引用才能使用value然后push进去
//     }

//     let mut my_vec = vec![];
//     let val1 = 1;
//     let val2 = 2;

//     let a = &mut my_vec;
//     insert_value(a, &val1);
//     println!("a is {:?}", a);

//     let b = &mut my_vec;
//     insert_value(b, &val2);
//     println!("b is {:?}", b);

//     println!("{my_vec:?}");
// }

//解法二：只留下一个'a
// fn test_lifetime_multiple(){
//     fn insert_value<'a>(my_vec:& mut Vec<i32>, value:&i32){
//         my_vec.push(*value); 
//     }

//     let mut my_vec: Vec<i32> = vec![];
//     let val1 = 1;
//     let val2 = 2;

//     let a = &mut my_vec;
//     insert_value(a, &val1);
//     println!("a is {:?}", a);

//     let b = &mut my_vec;
//     insert_value(b, &val2);
//     println!("b is {:?}", b);

//     println!("{my_vec:?}");
// }

//三，但没区别
fn test_lifetime_multiple(){
    fn insert_value<'a,'b>(my_vec:&'a mut Vec<i32>, value:&'b i32)
    where
    'b:'a  //生命周期'b必须大于等于'a,不加也能运行，显得好像多此一举。
    {
        my_vec.push(*value); 
    }

    let mut my_vec: Vec<i32> = vec![];
    let val1 = 1;
    let val2 = 2;

    let a = &mut my_vec;
    insert_value(a, &val1);
    println!("a is {:?}", a);

    let b = &mut my_vec;
    insert_value(b, &val2);
    println!("b is {:?}", b);

    println!("{my_vec:?}");
}
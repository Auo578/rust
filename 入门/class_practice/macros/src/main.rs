//1使用声明宏
//过程宏,不太熟练。
//关于宏方面应该再多看看


//（两种导入方法？）
// #[macro_use]
// extern crate dec;

use::dec::{repeat,sum,max_value};

fn main() {
    assert_eq!(repeat!("x",3) ,"xxx"); 
    assert_eq!(sum!(1,2,3, 4, 5),15); 
    assert_eq!(max_value!(1,8,9),9);
}


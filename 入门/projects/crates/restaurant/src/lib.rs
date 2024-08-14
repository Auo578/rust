// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }




// pub fn eat_at_restaurant() {
//     // 绝对路径
//     crate::front_of_house::hosting::add_to_waitlist();

//     // 相对路径
//     front_of_house::hosting::add_to_waitlist();
// }

fn deliver_order(){}

// mod back_of_house{
//     fn fix_incorrect_order(){
//         cook_order();
//         super::deliver_order();
//     }

//     fn cook_order(){}
// }


// mod back_of_house {
//     pub struct Breakfast {
//         pub toast: String,
//         seasonal_fruit: String,
//     }

//     impl Breakfast {
//         pub fn summer(toast: &str) -> Breakfast {
//             Breakfast {
//                 toast: String::from(toast),
//                 seasonal_fruit: String::from("peaches"),
//             }
//         }
//     }
// }

// pub fn eat_at_restaurant() {
//     // 在夏天订购一个黑麦土司作为早餐
//     let mut meal = back_of_house::Breakfast::summer("Rye");
//     // 改变主意更换想要面包的类型
//     meal.toast = String::from("Wheat");
//     println!("I'd like {} toast please", meal.toast);

//     // 如果取消下一行的注释代码不能编译；
//     // 不允许查看或修改早餐附带的季节水果
//     // meal.seasonal_fruit = String::from("blueberries");
// }


mod back_of_house{
    pub enum  Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant(){
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

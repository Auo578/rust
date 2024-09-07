// fn main() {
//     box_study();
// }

// fn box_study() {
//     // let a = Box::new(5);
//     // println!("{}",a);

//     // let s :Box<str> = "Hello".into();
//     // println!("{}",s);

//     // let arr:Box<[i32]> = vec![1,2,3].into_boxed_slice();
//     // println!("{:?}",arr);

//     // let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));

//     // // let animal = vec![Dog,Cat];
//     // let animals: Vec<Box<dyn animal>> = vec![Box::new(Dog),Box::new(Cat)];
//     // for animal in animals.iter(){
//     //     animal.speak();
//     //     animal.desc();
//     // }
// }
//     pub trait Deref {
//         type Target:? Sized;
//         fn deref(&self) -> &Self::Target;
//     }

// enum List{
//     Cons(i32,Box<List>),
//     Nil,
// }

// trait animal {
//     fn speak(&self);
//     fn desc(&self);
// }

// struct Dog;
// struct Cat;

// impl animal for Dog {
//     fn speak(&self) {
//         println!("woof");
//     }

//     fn desc(&self) {
//         println!("I'm a dog");
//     }
// }

// impl animal for Cat {
//     fn speak(&self) {
//         println!("meow");
//     }

//     fn desc(&self) {
//         println!("I'm a cat");
//     }
// }


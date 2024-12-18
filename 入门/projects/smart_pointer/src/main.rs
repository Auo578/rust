// // struct CustomSmartPointer {
// //     data: String,
// // }

// // impl Drop for CustomSmartPointer {
// //     fn drop(&mut self) {
// //         println!("Dropping CustomSmartPointer with data `{}`!", self.data);
// //     }
// // }

// // fn main() {
// //     let c = CustomSmartPointer {
// //         data: String::from("my stuff"),
// //     };
// //     let d = CustomSmartPointer {
// //         data: String::from("other stuff"),
// //     };
// //     println!("CustomSmartPointers created.");
// // }

// pub trait Messenger {
//     fn send(&self, msg: &str);
// }

// pub struct LimitTracker<'a, T: Messenger> {
//     messenger: &'a T,
//     value: usize,
//     max: usize,
// }

// impl<'a, T> LimitTracker<'a, T>
// where
//     T: Messenger,
// {
//     pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
//         LimitTracker {
//             messenger,
//             value: 0,
//             max,
//         }
//     }

//     pub fn set_value(&mut self, value: usize) {
//         self.value = value;

//         let percentage_of_max = self.value as f64 / self.max as f64;

//         if percentage_of_max >= 1.0 {
//             self.messenger.send("Error: You are over your quota!");
//         } else if percentage_of_max >= 0.9 {
//             self.messenger
//                 .send("Urgent warning: You've used up over 90% of your quota!");
//         } else if percentage_of_max >= 0.75 {
//             self.messenger
//                 .send("Warning: You've used up over 75% of your quota!");
//         }
//     }
// }

// // #[cfg(test)]
// // mod tests {
// //     use super::*;

// //     struct MockMessenger {
// //         sent_messages: Vec<String>,
// //     }

// //     impl MockMessenger {
// //         fn new() -> MockMessenger {
// //             MockMessenger {
// //                 sent_messages: vec![],
// //             }
// //         }
// //     }

// //     impl Messenger for MockMessenger {
// //         fn send(&self, message: &str) {
// //             self.sent_messages.push(String::from(message));
// //         }
// //     }

// //     #[test]
// //     fn it_sends_an_over_75_percent_warning_message() {
// //         let mock_messenger = MockMessenger::new();
// //         let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

// //         limit_tracker.set_value(80);

// //         assert_eq!(mock_messenger.sent_messages.len(), 1);
// //     }
// // }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::cell::RefCell;

//     struct MockMessenger {
//         sent_messages: RefCell<Vec<String>>,
//     }

//     impl MockMessenger {
//         fn new() -> MockMessenger {
//             MockMessenger {
//                 sent_messages: RefCell::new(vec![]),
//             }
//         }
//     }

//     // impl Messenger for MockMessenger {
//     //     fn send(&self, message: &str) {
//     //         let mut one_borrow = self.sent_messages.borrow_mut();
//     //         let mut two_borrow = self.sent_messages.borrow_mut();

//     //         one_borrow.push(String::from(message));
//     //         two_borrow.push(String::from(message));
//     //     }
//     // }

//     impl Messenger for MockMessenger {
//         fn send(&self, message: &str) {
//             self.sent_messages.borrow_mut().push(String::from(message));
//         }
//     }

//     #[test]
//     fn it_sends_an_over_75_percent_warning_message() {
//         let mock_messenger = MockMessenger::new();
//         let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

//         limit_tracker.set_value(80);

//         assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
//     }
// }

// use crate::List::{Cons, Nil};
// use std::cell::RefCell;
// use std::rc::Rc;

// #[derive(Debug)]
// enum List {
//     Cons(i32, RefCell<Rc<List>>),
//     Nil,
// }

// impl List {
//     fn tail(&self) -> Option<&RefCell<Rc<List>>> {
//         match self {
//             Cons(_, item) => Some(item),
//             Nil => None,
//         }
//     }
// }

// fn main() {
//     let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

//     println!("a initial rc count = {}", Rc::strong_count(&a));
//     println!("a next item = {:?}", a.tail());

//     let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

//     println!("a rc count after b creation = {}", Rc::strong_count(&a));
//     println!("b initial rc count = {}", Rc::strong_count(&b));
//     println!("b next item = {:?}", b.tail());

//     if let Some(link) = a.tail() {
//         *link.borrow_mut() = Rc::clone(&b);
//     }

//     println!("b rc count after changing a = {}", Rc::strong_count(&b));
//     println!("a rc count after changing a = {}", Rc::strong_count(&a));

//     // Uncomment the next line to see that we have a cycle;
//     // it will overflow the stack
//     // println!("a next item = {:?}", a.tail());
// }

use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

// fn main() {
//     let leaf = Rc::new(Node {
//         value: 3,
//         parent: RefCell::new(Weak::new()),
//         children: RefCell::new(vec![]),
//     });

//     println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

//     let branch = Rc::new(Node {
//         value: 5,
//         parent: RefCell::new(Weak::new()),
//         children: RefCell::new(vec![Rc::clone(&leaf)]),
//     });

//     *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

//     println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
// }

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
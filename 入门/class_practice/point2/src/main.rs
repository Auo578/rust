use std::cell::RefCell;
use std::rc::{Rc, Weak};


#[derive(Debug)]
struct User {
    name: String,
    friend_list: RefCell<Vec<Weak<RefCell<User>>>>, 
}

impl User {
  
    fn new(name: String) -> Rc<RefCell<User>> {
        Rc::new(RefCell::new(User {
            name,
            friend_list: RefCell::new(Vec::new()),
        }))
    }

  
    fn add_friend(user: &Rc<RefCell<User>>, friend: &Rc<RefCell<User>>) { 
        user.borrow_mut().friend_list.borrow_mut().push(Rc::downgrade(friend));
        friend.borrow_mut().friend_list.borrow_mut().push(Rc::downgrade(user));
    }

    fn show_friends(user: &Rc<RefCell<User>>) {
        let user_borrow = user.borrow();
        println!("{}'s friends:", user_borrow.name);
        for friend_weak in user_borrow.friend_list.borrow().iter() {
            if let Some(friend_rc) = friend_weak.upgrade() {
                println!("User:{}", friend_rc.borrow().name);
            }
        }
    }
}

fn main() {
    let user1 = User::new("A".to_string());
    let user2 = User::new("B".to_string());
    let user3 = User::new("C".to_string());

    User::add_friend(&user1, &user2);
    User::add_friend(&user1, &user3);

    User::show_friends(&user1);
    User::show_friends(&user2);
    User::show_friends(&user3);
}



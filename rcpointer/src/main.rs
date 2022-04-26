use std::borrow::{Borrow, BorrowMut};
use std::cell::{Ref, RefCell};
use std::f32::consts::E;
use std::ops::Deref;
use std::rc::{Rc, Weak};

// #[derive(Debug)]
// enum List {
//     Cos(i32, RefCell<Rc<List>>),
//     Nic,
// }
//
// impl List {
//     fn tail(&self) -> Option<&RefCell<Rc<List>>> {
//         match self {
//             Cos(_, val) => Some(val),
//             Nic => None
//         }
//     }
// }


//
// use List::{Cos, Nic};

struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    child: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    // let a = Rc::new(Cos(5, RefCell::new(Rc::new(Nic))));
    // println!("initial count -> {}", Rc::strong_count(&a));
    // println!("a next item i = {:?}", a.tail());
    //
    // let b = Rc::new(Cos(10, RefCell::new(Rc::clone(&a))));
    //
    // println!("a rc count after b creation -> {}", Rc::strong_count(&a));
    // println!("b initial count is -> {}", Rc::strong_count(&b));
    // println!("b next item = {:?}", b.tail());
    //
    //
    // if let Some(link) = a.tail() {
    //     *link.borrow_mut() = Rc::clone(&b);
    // }
    //
    // println!("a rc then -> {}", Rc::strong_count(&a));
    // println!("b rc then -> {}", Rc::strong_count(&b));

    let leaf = Rc::new(Node {
        value: 1,
        parent: RefCell::new(Weak::new()),
        child: RefCell::new(vec![]),
    });

    println!("leaf strong rc -> {}", Rc::strong_count(&leaf));

    println!("------------------------");
    // {
        let branch = Rc::new(Node {
            value: 2,
            parent: RefCell::new(Weak::new()),
            child: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        println!("leaf strong rc -> {}", Rc::strong_count(&leaf));
        println!("branch strong rc -> {}", Rc::strong_count(&branch));

        println!("------------------------");

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!("leaf strong rc -> {}", Rc::strong_count(&leaf));
        println!("branch strong rc -> {}", Rc::strong_count(&branch));
        println!("leaf weak rc -> {}", Rc::weak_count(&leaf));
        println!("branch weak rc -> {}", Rc::weak_count(&branch));
    // }

    println!("------------------------");

    println!("leaf strong rc -> {}", Rc::strong_count(&leaf));
    println!("leaf weak rc -> {}", Rc::weak_count(&leaf));

    // let x = leaf.parent.borrow().upgrade();
    match leaf.parent.borrow().upgrade() {
        None => println!("parent does not exist"),
        Some(v) => println!("parent exists with value: {}", v.value)
    }

    println!("branch strong rc -> {}", Rc::strong_count(&branch));

}

enum List {
    Cos(i32, Box<List>),
    Nic,
}

use std::borrow::Borrow;
use std::ops::Deref;
use List::{Cos, Nic};

fn main() {
    let mut l = Cos(1, Box::new(Cos(2, Box::new(Nic))));


    loop {
        match l {
            Cos(i, b) => {
                println!("{}", i);
                l = *b;
            }
            Nic => {
                break;
            }
        }
    }

}


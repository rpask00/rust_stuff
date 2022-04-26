extern crate core;

#[allow(unused_imports)]
use std::fs::{self, File};
use std::io;
use std::io::{Read};
#[allow(unused_imports)]
use std::io::Error;

#[allow(unused_imports)]
use std::io::ErrorKind;


struct Powww {
    val: u32,
}

impl Powww {
    fn new(val: u32) -> Powww {
        Powww {
            val,
        }
    }
}

impl Iterator for Powww {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.val += 1;

        if self.val % 2 == 1 {
            return Some(-1);
        }

        if self.val > 20 {
            return None;
        }

        Some(self.val as i32)
    }
}

fn main() {
    let mut p = Powww::new(12);
    for v in p {
        println!("{}", v);
    }
}



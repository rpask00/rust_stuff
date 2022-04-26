pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}


fn main() {
    // let mut x = 34;
    // let y = "23";
    //
    //
    // while x > 1 {
    //     let x = match y.parse::<u32>() {
    //         Ok(v) => v,
    //         Err(e) => continue
    //     };
    // }

    let f = |x: u32| -> u32 {
        return x*2;
    };

    println!("{}", takeClosure(f,12));
}


fn takeClosure(f: fn(u32) -> u32, x: u32) -> u32 {
    f(x)
}
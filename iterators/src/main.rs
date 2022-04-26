struct Counter {
    count: u32,
}

impl Counter {
    pub fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            return Some(self.count);
        }
        None
    }
}

fn main() {
    let mut c = Counter::new();

    assert_eq!(c.next(), Some(1));
    assert_eq!(c.next(), Some(2));
    assert_eq!(c.next(), Some(3));
    assert_eq!(c.next(), Some(4));
    assert_eq!(c.next(), Some(5));
    assert_eq!(c.next(), None);
    let a = vec![1, 2, 3];
    let mut b = vec![1, 2, 3];
    let mut c = vec![1, 2, 3];
    let d = vec![1, 2, 3];
    // examplea(a);
    // exampleb(&mut b);
    examplec(&c);
    // println!("{}", b[1]);
    println!("{}", c[1]);
}

fn examplea(v: Vec<u32>) {
    for i in 0..v.len() {
        println!("{}", v[i]);
    }
}

fn exampleb(v: &mut Vec<u32>) {
    for i in 0..v.len() {
        v[i] = v[i] * 2;
        println!("{}", v[i]);
    }
}

fn examplec(v: &Vec<u32>) {
    for i in 0..v.len() {
        v[i] = v[i] * 2;
        println!("{}", v[i]);
    }
}

fn exampled(v: &Vec<u32>) {
    for i in 0..v.len() {
        // v[i] = v[i] * 2;
        println!("{}", v[i]);
    }
}


#[test]
fn using_other_trait_methods() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|a| a % 3 == 0)
        .sum();

    assert_eq!(sum, 183);
}
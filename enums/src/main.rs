fn main() {
    match add_value(Some(3), 5) {
        Some(i) => println!("{}", i),
        _ => ()
    }

    let mut str = get_str();
    print!("{}",str);
}


fn add_value(one: Option<u32>, add_value: u32) -> Option<u32> {
    match one {
        Some(i) => Some(i + add_value),
        _ => None,
    }
}

fn get_str() -> String {
    return String::from("under denver");
}
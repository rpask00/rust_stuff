fn main() {
    let auth_status: Option<&str> = None;
    let is_admin = false;
    let group_id: Result<u8, _> = "12".parse();

    if let Some(status) = auth_status {
        println!("status: {}", status);
    } else if is_admin {
        println!("auth status is adin");
    } else if let Ok(gid) = group_id {
        println!("auth_status: {}", gid);
    }


    let mut stack = Vec::new();

    stack.push(-1);
    stack.push(-2);
    stack.push(-3);

    while let Some(top) = stack.pop() {
        println!("top value: {}", top);
    }

    let (a, b, c, d) = ("asddf", 34, -12, false);


    let r = 10;

    let x = Some(12);
    match x {
        None => {}
        Some(12) => println!("got exactly 12"),
        Some(v) => println!("got val {}", v)
    }

    let num = Some(35);

    match num {
        Some(x) if x > 40 => println!("1 match"),
        Some(x) if x <=40 => println!("2 match"),
        Some(_) => {}
        None => {}
    }
}


fn add_one() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

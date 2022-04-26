#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,


}

impl User{
    fn sayHello(&self){
        println!("{}",self.username)
    }
}

impl User{
    fn say_goodbye(){
        println!("bye")
    }
}


fn main() {
    let user1 = User {
        email: String::from("abc@abc.pl"),
        username: String::from("jon"),
        sign_in_count: 12,
        active: true,
    };

    let user2 = User {
        username: String::from("sam"),
        ..user1
    };

    user2.sayHello();
    User::say_goodbye();

    pr


}

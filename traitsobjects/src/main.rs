use traitsobjects::{Button, Draw, Screen};

struct Input {
    label: String,
}

impl Draw for Input {
    fn draw(&self) {
        println!("drawing input");
    }
}


fn main() {
    let screen = Screen {
        components: vec![
            Box::new(Input {
                label: String::from("imie")
            }),
            Box::new(Button {
                width: 20,
                height: 30,
                label: String::from("dodaj"),
            }),
        ]
    };

    screen.run();

    // let screen2
}

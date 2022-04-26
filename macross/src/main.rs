use hello_macro::HeloMacro;
use hello_macro_derive::HeloMacro


#[derive(HelloMacro)]
struct Pancakes;



fn main(){
    Pancakes::hello_macro();
}


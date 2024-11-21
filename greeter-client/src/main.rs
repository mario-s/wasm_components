mod bindings;
use bindings::component::greeter::greet::get_greeting_message;
use bindings::component::even_odd::random_demo::is_even;
fn main() {
    println!("{}", get_greeting_message());
    if is_even() {
        println!("Is even");
    } else {
        println!("Is odd");
    }
}
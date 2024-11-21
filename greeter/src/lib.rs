#[allow(warnings)]
mod bindings;

use crate::bindings::exports::component::greeter::greet::Guest;

struct Component;

impl Guest for Component {
    /// Say hello!
    fn get_greeting_message() -> String {
        "Hello, World!".to_string()
    }
}

bindings::export!(Component with_types_in bindings);

mod bindings;
use bindings::exports::component::even_odd::random_demo::Guest;
use bindings::wasi::random::insecure::get_insecure_random_u64;
struct Component;
impl Guest for Component {
    /// Say hello!
    fn is_even() -> bool {
        let rand = get_insecure_random_u64();
        rand % 2 == 0
    }
}
bindings::export!(Component with_types_in bindings);
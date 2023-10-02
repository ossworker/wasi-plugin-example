cargo_component_bindings::generate!({});

use bindings::Guest;

struct Component;

impl Guest for Component {
    /// Say hello!
    fn run() {
        bindings::test::plugin::logger::log("sss");
        println!("222")
    }
}

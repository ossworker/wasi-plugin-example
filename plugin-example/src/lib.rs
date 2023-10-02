wit_bindgen::generate!({
    world: "plugin",
    path: "../plugin-host/plugin.wit",
    exports: {
        world: MyHost,
    }
});

struct MyHost;

impl Guest for MyHost {
    fn run() {
        test::plugin::logger::log("plugin log");
        print!("hello, world!");
    }
}

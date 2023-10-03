use anyhow::Ok;
use wasmtime::component::{Component, Linker};
use wasmtime::Store;
use wasmtime_wasi::preview2::Table;
use wasmtime_wasi::preview2::WasiCtx;
use wasmtime_wasi::preview2::WasiCtxBuilder;
use wasmtime_wasi::preview2::WasiView;

wasmtime::component::bindgen!({
    path: "./plugin.wit",
    world: "plugin",
    async: true,

});

// const PLUGIN_FILE: &str = "../plugin-example/target/wasm32-wasi/debug/plugin_example.wasm";

// const PLUGIN_FILE: &str = "../plugin-example/demo.component.wasm";
// const PLUGIN_FILE: &str = "../plugin-component/plugin.wasm";

struct SimpleLogger {}

struct SimplePluginCtx {
    logger: SimpleLogger,
    table: Table,
    context: WasiCtx,
}

impl WasiView for SimplePluginCtx {
    fn table(&self) -> &Table {
        &self.table
    }

    fn table_mut(&mut self) -> &mut Table {
        &mut self.table
    }

    fn ctx(&self) -> &WasiCtx {
        &self.context
    }

    fn ctx_mut(&mut self) -> &mut WasiCtx {
        &mut self.context
    }
}

#[async_trait::async_trait]
impl test::plugin::logger::Host for SimpleLogger {
    async fn log(&mut self, msg: String) -> wasmtime::Result<String> {
        println!("{}", msg);

        Ok(String::from("ok"))
    }
}

#[tokio::main]
async fn call_wasm(plugin_file: &str, input: serde_json::value::Value) {
    let str = input.get("name").unwrap().as_str().unwrap();
    println!("{}", str);

    let mut engine_config = wasmtime::Config::new();
    engine_config.wasm_component_model(true);
    engine_config.async_support(true);
    engine_config.wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable);
    engine_config.debug_info(true);

    let engine = wasmtime::Engine::new(&engine_config).unwrap();

    let mut linker: Linker<SimplePluginCtx> = Linker::new(&engine);

    wasmtime_wasi::preview2::command::add_to_linker(&mut linker).unwrap();

    Plugin::add_to_linker(&mut linker, |state: &mut SimplePluginCtx| &mut state.logger).unwrap();

    let table = wasmtime_wasi::preview2::Table::new();

    let wasi_ctx = WasiCtxBuilder::new()
        .inherit_stdin()
        .inherit_stdout()
        .inherit_stderr()
        .build();

    let mut store = Store::new(
        &engine,
        SimplePluginCtx {
            logger: SimpleLogger {},
            table,
            context: wasi_ctx,
        },
    );

    let component = Component::from_file(&engine, plugin_file).expect("could not find plugin");

    let (plugin, _instance) = Plugin::instantiate_async(&mut store, &component, &linker)
        .await
        .expect("could not instantialte plugin");

    plugin.call_run(&mut store).await.unwrap();
}

#[cfg(test)]
mod test_plugin {

    use super::*;

    const PLUGIN_FILE: &str = "../plugin-component/plugin.wasm";

    #[test]
    fn test_call_wasm() {
        let json = r#"
        {
            "name": "张三",
            "age": 33,
            "pet_phrase": [
                "Bond, James Bond.",
                "Shaken, not stirred."
            ]
        }"#;
        // let mut map: HashMap<&str, Value> = HashMap::new();
        // map.insert("id", Value::String("111".to_string()));
        // map.insert("bytes", serde_json::to_value(json).unwrap());
        // let map_param = serde_json::to_vec(&map).unwrap();
        let params: serde_json::value::Value = serde_json::from_str(json).unwrap();
        println!("{:#?}", params);
        call_wasm(PLUGIN_FILE, params);
    }
}

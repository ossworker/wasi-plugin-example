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

const PLUGIN_FILE: &str = "../plugin-example/demo.component.wasm";

struct SimpleLogger {}

struct SimplePluginCtx {
    logger: SimpleLogger,
    table: Table,
    context: WasiCtx,
    http: WasiHttpCtx,
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

impl WasiHttpView for SimplePluginCtx {
    fn http_ctx(&self) -> &WasiHttpCtx {
        &self.http
    }
    fn http_ctx_mut(&mut self) -> &mut WasiHttpCtx {
        &mut self.http
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
async fn main() {
    let mut engine_config = wasmtime::Config::new();
    engine_config.wasm_component_model(true);
    engine_config.async_support(true);
    engine_config.wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable);
    engine_config.debug_info(true);

    let engine = wasmtime::Engine::new(&engine_config).unwrap();

    let mut linker: Linker<SimplePluginCtx> = Linker::new(&engine);

    wasmtime_wasi::preview2::command::add_to_linker(&mut linker).unwrap();

    Plugin::add_to_linker(&mut linker, |context| &mut context.logger).unwrap();

    let mut table = wasmtime_wasi::preview2::Table::new();

    let wasi_ctx = WasiCtxBuilder::new()
        .build(&mut table)
        .expect("Could not build WASI context");

    let http = WasiHttpCtx::new();

    let mut store = Store::new(
        &engine,
        SimplePluginCtx {
            logger: SimpleLogger {},
            table,
            context: wasi_ctx,
            http,
        },
    );

    let component = Component::from_file(&engine, PLUGIN_FILE).expect("could not find plugin");

    let (plugin, _instance) = Plugin::instantiate_async(&mut store, &component, &linker)
        .await
        .expect("could not instantialte plugin");

    plugin.call_run(&mut store).await.unwrap();
}

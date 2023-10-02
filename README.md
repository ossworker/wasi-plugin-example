# wasmtime wasi component

* plugin-component  cargo-component
* plugin-example  wit+ cargo wasm32-wasi  + wasm-tools
* plugin-host  runtime


> plugin-example
wasm-tools component new ./target/wasm32-wasi/debug/plugin_example.wasm -o demo.component.wasm --adapt ./wasi_snapshot_preview1.reactor.wasm



  

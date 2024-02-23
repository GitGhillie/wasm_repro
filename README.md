Build with `cargo build --release --target wasm32-unknown-unknown`.
Then `wasm-bindgen --no-typescript --out-name wasm_repro --out-dir wasm --target web target/wasm32-unknown-unknown/release/wasm_repro.wasm`.
Go to wasm folder and start server with something like `live-server`.

The app will not load, with error `Uncaught TypeError: Failed to resolve module specifier "env". Relative references must start with either "/", "./", or "../"`.
The app will load normally after commenting out `.insert_resource(GlobalRng::new())`.
Or by changing Bevy to 0.12 and Bevy_turborand to 0.7.

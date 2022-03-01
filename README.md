# Birds And Crustaceans


Build release server
```sh
cargo build --bin server --release
```

Build release client
```sh
cargo build --bin client --release --target wasm32-unknown-unknown
wasm-bindgen --out-name birds_and_crustaceans --out-dir wasm/target --target web target/wasm32-unknown-unknown/release/client.wasm
```

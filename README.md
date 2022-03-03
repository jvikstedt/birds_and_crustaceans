# Birds And Crustaceans

Deterministic rollback based networking game made with Bevy engine.

## Build & Dev

Dev server
```sh
make dev-server
```

Dev client
```sh
make dev-client
```

Build release server
```sh
cargo build --bin server --release
```

Build release client
```sh
cargo build --bin client --release --target wasm32-unknown-unknown
wasm-bindgen --out-name birds_and_crustaceans --out-dir wasm/target --target web target/wasm32-unknown-unknown/release/client.wasm
```

## Game image

![alt text](https://github.com/jvikstedt/birds_and_crustaceans/blob/main/birds_and_crustaceans.png?raw=true)

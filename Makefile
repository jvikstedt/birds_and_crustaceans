dev-client:
	cargo build --bin client --target wasm32-unknown-unknown && wasm-bindgen --out-name birds_and_crustaceans --out-dir wasm/target --target web target/wasm32-unknown-unknown/debug/client.wasm && basic-http-server wasm

dev-server:
	cargo run --bin server

release-client:
	cargo build --bin client --release --target wasm32-unknown-unknown && wasm-bindgen --out-name birds_and_crustaceans --out-dir wasm/target --target web target/wasm32-unknown-unknown/release/client.wasm && scp -r wasm/* root@104.131.91.47:/var/www/gaminghouse.io/html

release-server:
	cargo build --bin server --release && scp target/release/server root@104.131.91.47:

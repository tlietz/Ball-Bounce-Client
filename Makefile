
default: clean rustup release server

release: clean rustup
	mkdir web
	cargo build --target wasm32-unknown-unknown --release
	cargo test
	cp -rf target/wasm32-unknown-unknown/release/* web/

debug: clean rustup
	mkdir web
	cargo build --target wasm32-unknown-unknown 
	cargo test
	cp -rf target/wasm32-unknown-unknown/debug/* web/

rustup: 
	rustup target add wasm32-unknown-unknown

server: 
	basic-http-server .
clean:
	rm -rf web
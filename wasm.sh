rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release && cp target/wasm32-unknown-unknown/release/tofuwabohu.wasm . && python -m http.server 8000
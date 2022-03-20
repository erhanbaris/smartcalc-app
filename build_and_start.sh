cargo build --release --lib --target wasm32-unknown-unknown 
wasm-bindgen target/wasm32-unknown-unknown/release/smartcalc_app.wasm --out-dir docs --no-modules --no-typescript
wasm-gc docs/smartcalc_app_bg.wasm
(cd docs && basic-http-server --addr 0.0.0.0:9090 .)
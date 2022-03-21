@echo off

@REM Starts a local web-server that serves the contents of the `doc/` folder,
@REM which is the folder to where the web version is compiled.

cargo build --release --lib --target wasm32-unknown-unknown 
wasm-bindgen target/wasm32-unknown-unknown/release/smartcalc_app.wasm --out-dir docs --no-modules --no-typescript
wasm-gc docs/smartcalc_app_bg.wasm

(cd docs && basic-http-server --addr 127.0.0.1:8080 .)
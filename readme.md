# SmartCalc-App


## Build desktop applications

```bash
npm install

# For windows
npm run pack:win64
npm run pack:win32

# For macos
npm run pack:osx
```

## Build and start as a web application

```bash
cargo build --target wasm32-unknown-unknown --release
wasm-pack build --out-dir src/js/ --target web --no-typescript
wasm-gc src/js/libsmartcalc_bg.wasm
python -m SimpleHTTPServer
```
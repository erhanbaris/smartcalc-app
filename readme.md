<p align="center">
  <img src="assets/smartcalc.png" alt="SmartCalc" width="150" height="150" />
</p>

# SmartCalc-App
This project is for building and generating SmartCalc web and desktop applications.
You can find original **smartcalc** library source code at [here](https://github.com/erhanbaris/smartcalc/).

## Web application
[Open web application](https://erhanbaris.github.io/smartcalc-app/)


## Windows binaries
[Download x64 application](https://github.com/erhanbaris/smartcalc/releases/download/v1.0.1/smartcalc-win64-v1.0.1.zip)

[Download x86 application](https://github.com/erhanbaris/smartcalc/releases/download/v1.0.1/smartcalc-win32-v1.0.1.zip)


## Macos binary
[Download application](https://github.com/erhanbaris/smartcalc/releases/download/v1.0.1/smartcalc-osx-v1.0.1.zip)



## Build desktop applications

```bash
npm install

# For windows
npm run pack:win64
npm run pack:win32

# For macos
npm run pack:osx
```

## Start web application locally

```bash
cargo build --target wasm32-unknown-unknown --release
wasm-pack build --out-dir src/js/ --target web --no-typescript
wasm-gc src/js/smartcalc_app_bg.wasm
python -m SimpleHTTPServer
```

Then open http://127.0.0.1:8000/ at your favorite browser.

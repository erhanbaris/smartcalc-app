<p align="center">
  <img src="assets/smartcalc.png" alt="SmartCalc" width="150" height="150" />
</p>

# SmartCalc-App
This project is for building and generating SmartCalc web and desktop applications.
You can find original **smartcalc** library source code at [here](https://github.com/erhanbaris/smartcalc/).

## Web application
[Open web application](https://erhanbaris.github.io/smartcalc-app/)


## Start web application locally
Dependencies:
```bash
cargo install basic-http-server
cargo install wasm-bindgen-cli
cargo install wasm-pack
cargo install wasm-gc
```

For publish:
```bash
./build_and_start.sh
```

Then open http://127.0.0.1:8000/ at your favorite browser.

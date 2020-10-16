# Rust LVGL Bindings for Mynewt on PineTime Smart Watch

This crate contains LVGL Bindings used by the PineTime Watch Face Framework: [`pinetime-watchface`](https://crates.io/crates/pinetime-watchface)

Check out a sample watch face created with this framework: [`barebones-watchface`](https://crates.io/crates/barebones-watchface)

The LVGL Bindings are compatible with PineTime Mynewt Firmware [`pinetime-rust-mynewt`](https://github.com/lupyuen/pinetime-rust-mynewt) and with PineTime WebAssembly Simulator [`lvgl-wasm`](https://github.com/AppKaki/lvgl-wasm/tree/mynewt)

Refer to the article...

["Create Your Own PineTime Watch Face in Rust... And Publish on crates.io"](https://lupyuen.github.io/pinetime-rust-mynewt/articles/watchface)

To generate bindings and publish to crates.io...

```bash
scripts/gen-bindings.sh

cargo package --list --allow-dirty

cargo publish --dry-run --allow-dirty

cargo publish    
```

See [`src`](src) for more details.

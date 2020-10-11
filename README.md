# Rust LVGL Bindings for Mynewt on PineTime Smart Watch

This crate contains LVGL Bindings used by the PineTime Watch Face Framework: [`pinetime-watchface`](https://crates.io/crates/pinetime-watchface)

Check out a sample watch face created with this framework: [`barebones-watchface`](https://crates.io/crates/barebones-watchface)

Refer to the article...

["Bluetooth Time Sync, Rust Watch Faces and LVGL on PineTime Mynewt"](https://lupyuen.github.io/pinetime-rust-mynewt/articles/timesync)

See [`src`](src) for more details.

To publish to crates.io...

```bash
cargo package --list --allow-dirty

cargo publish --dry-run --allow-dirty

cargo publish    
```
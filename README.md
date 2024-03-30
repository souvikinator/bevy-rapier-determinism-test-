```bash
cargo build --release --target wasm32-unknown-unknown
```

```bash
wasm-bindgen --out-dir ./out/ --target web .\target\wasm32-unknown-unknown\release\bevy-breakout.wasm
```

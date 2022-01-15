# countminsketch-rs

approximate streaming heavy hitters

to benchmark hash functions used
```bash
nightly rustup toolchain install nightly # install nightly channel (benches only come with non stable channel)
rustup run nightly cargo bench # benchmark non-criptographic hash functions used
```

to test library
```bash
cargo test
```
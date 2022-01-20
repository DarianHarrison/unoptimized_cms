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




References
```
Gakhov, A. (2019). Probabilistic Data Structures and algorithms for Big Data Applications. Books on Demand.
Lu, F. (2021). rust-fasthash Version (0.4.0). rust-fasthash. Retrieved January 20, 2022, from https://github.com/flier/rust-fasthash. 
```

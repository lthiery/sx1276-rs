# sx1276-rs

## Dependencies
### Rust Dependency
This build requires the nightly toolchain in order to use: 
```
#![feature(rustc_private)]
```
Install nightly toolchain:
```
rustup toolchain install nightly
```

### Dependencies to build sx1276-sys
```
sudo apt-get install libclang-dev clang-8
```

## Build
```
cargo +nightly build
```
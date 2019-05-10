use bindgen;
use bindgen::builder;
use std::env;
use std::path::PathBuf;
use std::process::Command;


fn main() {
    // build `libloragw`
    Command::new("make")
        .args(&["-C", "radio/sx1276"])
        .status()
        .expect("sx1276 build failed");

    let radio_path =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("radio/");

    let conf_path =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("conf");

    println!(
        "cargo:rustc-link-search=native={}",
        radio_path.to_str().unwrap()
    );
    println!(
        "cargo:rustc-link-search=native={}",
        conf_path.to_str().unwrap()
    );
    println!("cargo:rustcpp-link-lib=static=sx1276");

      // make the bindings
   let bindings = bindgen::Builder::default()
       .clang_arg("-I./conf")
       .clang_arg("-I../embedded-hal-bindings/target")
       .header("embedded-hal-bindings.h")
       .header("config.h")
       .header("system/gpio.h")
       .header("conf/spi-board.h")
       .header("system/spi.h")
       .header("radio/radio.h")
       .header("radio/sx1276/sx1276.h")
       .whitelist_function("SX1276Reset")
       .trust_clang_mangling(false)
       .derive_copy(false)
       .layout_tests(false)
       .generate()
       .expect("Failed to generate sx1276 bindings!");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

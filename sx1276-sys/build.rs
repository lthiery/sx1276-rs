#![feature(rustc_private)]
extern crate cc;

use bindgen;
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

   // make the bindings
   let bindings = bindgen::Builder::default()
       .raw_line("use cty;")
       .use_core()
       .ctypes_prefix("cty")
       .header("board.h")
       .header("helium.h")
       .header("radio/radio.h")
       .header("radio/sx1276/sx1276.h")
       .whitelist_var("XTAL_FREQ")
       .whitelist_var("FREQ_STEP")
       .whitelist_var("RX_BUFFER_SIZE")
       .whitelist_type("RfEvent")
       .whitelist_type("RadioState")
       .whitelist_type("RadioModems")
       .whitelist_type("ClientEvent")
       .whitelist_type("QualityOfService")
       .whitelist_type("RfConfig")
       .whitelist_function("helium_rf_init")
       .whitelist_function("helium_rf_handle_event")
       .whitelist_function("helium_ping")
       .whitelist_function("helium_pong")
       .whitelist_function("SX1276Init")
       .whitelist_function("SX1276GetStatus")
       .whitelist_function("SX1276SetModem")
       .whitelist_function("SX1276SetChannel")
       .whitelist_function("SX1276IsChannelFree")
       .whitelist_function("SX1276Random")
       .whitelist_function("SX1276SetRxConfig")
       .whitelist_function("SX1276SetTxConfig")
       .whitelist_function("SX1276GetTimeOnAir")
       .whitelist_function("SX1276Send")
       .whitelist_function("SX1276SetSleep")
       .whitelist_function("SX1276SetStby")
       .whitelist_function("SX1276SetRx")
       .whitelist_function("SX1276StartCad")
       .whitelist_function("SX1276ReadRssi")
       .whitelist_function("SX1276Write")
       .whitelist_function("SX1276Read")
       .whitelist_function("SX1276WriteBuffer")
       .whitelist_function("SX1276ReadBuffer")
       .whitelist_function("SX1276SetMaxPayloadLength")
       .trust_clang_mangling(false)
       .rustfmt_bindings(true)
       .rustified_enum("ClientEvent")
       .rustified_enum("RfEvent")
       .rustified_enum("QualityOfService")
       .derive_copy(false)
       .layout_tests(false)
       .rustfmt_bindings(false)
       .generate()
       .expect("Failed to generate sx1276 bindings!");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    cc::Build::new()
        .pic(false)
        .include("../embedded-hal-bindings/target")
        .include("radio")
        .file("helium.c")
        .file("radio/sx1276/sx1276.c")
        .compile("sx1276");
}

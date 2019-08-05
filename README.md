# sx1276-rs / LongFi Driver

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

### Dependencies to build sx1276-sys (Ubuntu 18.04)
```
sudo apt-get install libclang-dev clang-8 libc6-dev-i386
```

## Build
```
cargo +nightly build
```

## Debug w/JLink
```
JLinkGDBServer -device STM32L072CZ -speed 4000 -if jtag -AutoConnect -1 -port 3333
```

## Cargo Config 
```
[target.thumbv6m-none-eabi]
# uncomment this to make `cargo run` execute programs on QEMU
# runner = "qemu-system-arm -cpu cortex-m3 -machine lm3s6965evb -nographic -semihosting-config enable=on,target=native -kernel"

[target.'cfg(all(target_arch = "arm", target_os = "none"))']
# uncomment ONE of these three option to make `cargo run` start a GDB session
# which option to pick depends on your system
# runner = "arm-none-eabi-gdb -q -x openocd.gdb"
# runner = "gdb-multiarch -q -x openocd.gdb"
# runner = "gdb -q -x openocd.gdb"
# runner = "gdb-multiarch -tui -q -x openocd.gdb"

rustflags = [
  # LLD (shipped with the Rust toolchain) is used as the default linker
  "-C", "link-arg=-Tlink.x",

  # if you run into problems with LLD switch to the GNU linker by commenting out
  # this line
  "-C", "linker=arm-none-eabi-ld",

  # if you need to link to pre-compiled C libraries provided by a C toolchain
  # use GCC as the linker by commenting out both lines above and then
  # uncommenting the three lines below
  # "-C", "linker=arm-none-eabi-gcc",
  # "-C", "link-arg=-Wl,-Tlink.x",
  # "-C", "link-arg=-nostartfiles",
]

[build]
# Pick ONE of these compilation targets
target = "thumbv6m-none-eabi"    # Cortex-M0 and Cortex-M0+
# target = "thumbv7m-none-eabi"    # Cortex-M3
# target = "thumbv7em-none-eabi"   # Cortex-M4 and Cortex-M7 (no FPU)
# target = "thumbv7em-none-eabihf" # Cortex-M4F and Cortex-M7F (with FPU
```
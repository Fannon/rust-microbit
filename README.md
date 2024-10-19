
## Setup

```sh
# https://doc.rust-lang.org/beta/rustc/platform-support/thumbv7em-none-eabi.html
rustup target add thumbv7em-none-eabihf 
rustup show

rustc --print=target-list


rustup component add llvm-tools
cargo install cargo-binutils
cargo size -- -Ax 
```

## Run

```sh
probe-rs list
probe-rs info --probe 0d28:0204

cargo embed --chip nRF52833_xxAA
cargo embed --chip nRF52833_xxAA --probe 0d28:0204
```


## Links

* https://www.youtube.com/watch?v=TOAynddiu5M
* https://doc.rust-lang.org/beta/rustc/platform-support.html
    * https://doc.rust-lang.org/beta/rustc/platform-support/thumbv7em-none-eabi.html
        * thumbv7em-none-eabihf
* https://www.youtube.com/watch?v=A9wvA_S6m7Y&t=14s
    * https://github.com/microbit-foundation/microbit-v2-hardware/blob/main/V2.21/MicroBit_V2.2.1_nRF52820%20schematic.PDF
    
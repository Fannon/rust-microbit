
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

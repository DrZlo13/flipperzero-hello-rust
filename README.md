# "Hello, Rust!" for the Flipper Zero

This is an example of how to build a Rust-based Flipper application that runs
from the SD-card.

This depends upon the Flipper Application SDK which should be included in the
upcoming 0.67 firmware release.

**Note:** This currently requires a firmware with patches from my
[`rust` branch of `flipper-firmware`](https://github.com/dcoles/flipperzero-firmware/tree/rust).

## Building

1. Install the `thumbv7em-none-eabihf` Rust target:
    ```
    rustup target add thumbv7em-none-eabihf
    ```
2. Clone the [`flipperzero-firmware`](https://github.com/flipperdevices/flipperzero-firmware) repository:
    ```
    git clone --recurse-submodules https://github.com/flipperdevices/flipperzero-firmware.git && cd flipperzero-firmware
    ```
3. Clone this repository into `applications_user`:
    ```
    git clone https://github.com/dcoles/flipperzero-hello-rust.git applications_user/hello_rust
    ```
4. Build the project as a Rust library:
    ```
    (cd applications_user/hello_rust && cargo build --release)
    ```
5. Built the Flipper Application Package:
    ```
    ./fbt firmware_hello_rust
    ```
6. Copy the package from `build/f7-firmware-D/hello_rust.fap` to `apps/Misc`
on the Flipper Zero's SD-card:
    ```
    ./scripts/storage.py mkdir /ext/apps/Misc
    ./scripts/storage.py send build/f7-firmware-D/hello_rust.fap /ext/apps/Misc/hello_rust.fap
    ```

## Build Tasks

This project uses [cargo-make](https://crates.io/crates/cargo-make) to help
automate some common tasks.

### `cargo make build`

Builds application by running `cargo build` and then `fbt`.

### `cargo make install`

Copy most recent build of application to connected Flipper Zero.

### `cargo make build-install`

Shorthand for `cargo make build` followed by `cargo make install`.

### `cargo make cli`

Connect to the Flipper Zero's serial command-line interface.

## License

Licensed under the MIT License. See [LICENSE](LICENSE) for details.

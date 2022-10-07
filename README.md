# "Hello, Rust!" for the Flipper Zero

This is an example of how to build a Rust-based Flipper application that runs
from the SD-card.

**Note:** This depends upon the Flipper Application SDK which is included in
the `0.67` release and some Rust-specific fixes which are included in `0.68.1` release.

## Building

1. Switch to nightly version of Rust 
    ```
    rustup default nightly
    ```
2. Install the `thumbv7em-none-eabihf` Rust target:
    ```
    rustup target add thumbv7em-none-eabihf
    ```
3. Clone the [`flipperzero-firmware`](https://github.com/flipperdevices/flipperzero-firmware) repository:
    ```
    git clone --recurse-submodules https://github.com/flipperdevices/flipperzero-firmware.git && cd flipperzero-firmware
    ```
4. Clone this repository into `applications_user`:
    ```
    git clone https://github.com/dcoles/flipperzero-hello-rust.git applications_user/hello_rust
    ```
5. Build the project as a Rust library:
    ```
    (cd applications_user/hello_rust && cargo build --release)
    ```
6. Build, upload and launch the app by executing:
    ```
    ./fbt launch_app APPSRC=applications_user/hello_rust
    ```
    Or do it manually by calling:
    ```
    ./fbt firmware_hello_rust
    ./scripts/storage.py mkdir /ext/apps/Misc
    ./scripts/storage.py send build/f7-firmware-D/hello_rust.fap /ext/apps/Misc/hello_rust.fap
    ```
    And after that launch the app on Flipper via `Menu->Applications->Misc->Hello, Rust`.

## Build Tasks

This project uses [cargo-make](https://crates.io/crates/cargo-make) to help
automate some common tasks.

### `cargo make build`

Builds application by running `cargo build` and then `fbt`.

### `cargo make install`

Copy most recent build of application to connected Flipper Zero.

### `cargo make build-install`

Shorthand for `cargo make build` followed by `cargo make install`.

### `cargo make build-run`

Build and launch most recent build of application on connected Flipper Zero.

### `cargo make cli`

Connect to the Flipper Zero's serial command-line interface.

## License

Licensed under the MIT License. See [LICENSE](LICENSE) for details.

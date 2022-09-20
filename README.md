# "Hello, Rust!" for the Flipper Zero

This is an example of how to build a Rust-based Flipper application that runs
from the SD-card.

This depends upon the Flipper Application SDK which should be included in the
upcoming 0.67 firmware release.

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
    git clone https://github.com/dcoles/flipper-hello-rust.git applications_user/hello_rust
    ```
4. Build the project as a Rust library:
    ```
    (cd applications_user/hello_rust && cargo build --release)
    ```
5. Built the Flipper Application Package:
    ```
    ./fbt firmware_hello_rust
    ```

Copy the package from `build/f7-firmware-D/hello_rust.fap` to `apps/Misc`
on the Flipper Zero's SD-card.
    ```
    ./scripts/storage.py mkdir /ext/apps/Misc
    ./scripts/storage.py send build/f7-firmware-D/hello_rust.fap /ext/apps/hello_rust.fap
    ```

## License

Licensed under the MIT License. See [LICENSE](LICENSE) for details.

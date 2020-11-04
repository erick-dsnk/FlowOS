# Flow OS

### Building the OS
**NOTE**: If this is your first time building it, you will need to first open a terminal in the folder with the source code and run `rustup override set nightly`. This will tell Rust to use the nightly version, since this uses some features only available in the nightly channel.

To build the app use the command:
```
cargo build --target flow_os.json
```
# Flow OS

### Building the OS
#### First Time
If this is your first time building it, you will need to first open a terminal in the folder with the source code and run `rustup override set nightly && rustup update nightly --force`. This will tell Rust to use the nightly version, since this uses some features only available in the nightly channel.

#### Additional requirements
To make an image you will also need to have `bootimage` installed.
Use this command to install it:
```
cargo install bootimage
```
You will also need the LLVM tools. Install using:
```
rustup component add llvm-tools-preview
```
Also install QEMU to be able to run it in a virtual machine.

#### Building
To build the OS and make a bootable image use the command:
```
cargo bootimage
```

The image will be located at `target/x86_64-flow_os/debug/bootimage-flow_os.bin`

Use QEMU to boot it up with the following command:
```
qemu-system-x86_64 -drive format=raw,file=target/x86_64-flow_os/debug/bootimage-flow_os.bin
```

**Additionally, you can use `cargo run` to build and boot Flow in QEMU**
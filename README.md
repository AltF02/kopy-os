# Kopy OS
Kopy os is a work in progress command line based operating system.

## How to run
* Install [qemu](https://www.qemu.org/download/) with `Add to path` selected
* Run `cargo run --release`

## Run natively
Requires an physical USB stick.
* Run `dd if=target/x86_64-blog_os/debug/bootimage-kopy_os.bin of=/dev/sdX && sync` where `/dev/sdX` is your device.
* Boot the USB
# UEFI driver example

Example UEFI driver:

- Written in Rust using [uefi-rs](https://github.com/rust-osdev/uefi-rs).
- Using UEFI driver model (`EFI_DRIVER_BINDING_PROTOCOL`)

## Running in QEMU

```
make qemu
```

The driver doesn't do anything, so it is only useful to inspect some properties
about it.

```
Shell> fs0:
FS0:\> load release\uefi-driver-example.efi
FS0:\> drivers
FS0:\> dh
FS0:\> unload -v -n 96
```

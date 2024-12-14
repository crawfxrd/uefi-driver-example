# UEFI driver example

Example UEFI driver:

- Using UEFI driver model (`EFI_DRIVER_BINDING_PROTOCOL`)
- Written in Rust using [uefi-rs](https://github.com/rust-osdev/uefi-rs).

```
make qemu
```

Loading the driver in QEMU:

```
Shell> fs0:
FS0:\> load release\uefi-driver-example.efi
```

Unloading the driver in QEMU:

```
FS0:\> dh
FS0:\> unload -v -n 96
```

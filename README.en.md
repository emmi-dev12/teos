# TeOS

> **[READ THE DISCLAIMER FIRST](DISCLAIMER.md)**  
> Experimental hobby kernel. **USB only.** Not Linux. Not macOS. Can destroy data. No warranty. Not affiliated with Apple.

[DE](README.md) · EN

TeOS is a **from-scratch** OS (not a Linux/Unix clone). Hardware target: **MacBook Pro 2018 15"** (`MacBookPro15,1`, T2 chip). Today we prove it in **QEMU**, then a USB stick. Dual-boot is **not** implemented.

## Status

v0.1: UEFI loader `BOOTX64.EFI` that paints `TeOS` on the GOP framebuffer.

## Build

```bash
rustup target add x86_64-unknown-uefi
./scripts/build.sh
./scripts/run-qemu.sh
```

## USB

Read **[DISCLAIMER.md](DISCLAIMER.md)** and **[docs/t2-boot.md](docs/t2-boot.md)**. Scripts refuse `disk0`.

## License

MIT. The disclaimer still applies.

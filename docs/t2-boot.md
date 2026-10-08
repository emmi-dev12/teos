# TeOS on MacBookPro15,1 (USB only)

Read [DISCLAIMER.md](../DISCLAIMER.md) first. Dual-boot is **not** supported.

Target: **MacBook Pro 2018 15-inch**, model **MacBookPro15,1**, T2 chip.

The name: [mate](mate.md).

## Allow external boot (you do this, once)

1. Shut down.
2. Power on and hold **Cmd-R** until Recovery.
3. Utilities → **Startup Security Utility**.
4. Allow **external media**. Set Secure Boot to **No Security** only if Apple’s screen requires it to pick EFI Boot.
5. Restart into macOS.

We do **not** bypass T2, dump keys, or touch the internal SSD.

## Flash a stick

```bash
diskutil list
# pick the USB. If it is disk0 you are wrong.
TEOS_DISK=diskN ./scripts/make-usb.sh
```

The script **exits if the id is disk0**.

ISO (same files): `./scripts/make-iso.sh` → `build/teos.iso`. GitHub Actions uploads that artifact. USB write is still **not disk0**.

## What is already on the stick (offline)

No extra download on the 15,1 for these:

- Boot film (mate hearts) + settings boxes
- Linux home (click or type)
- Kernel cmdline from the [t2linux basic setup](https://t2linux.org/guides/postinstall/): `intel_iommu=on iommu=pt pm_async=off`
- Public **apple_set_os** EFI protocol call (same idea as [0xbb/apple_set_os.efi](https://github.com/0xbb/apple_set_os.efi)) so the laptop may keep the Intel GPU awake. Harmless if the protocol is missing (QEMU).

## Kernel (already on the stick)

Boot uses the official **t2linux** kernel `6.18.54-1-t2-bookworm` ([T2-Debian-and-Ubuntu-Kernel](https://github.com/t2linux/T2-Debian-and-Ubuntu-Kernel), GPL-2). Fetched on the **build Mac**, then copied onto the USB. The 15,1 does not need the internet for that.

It includes t2bce (keyboard/trackpad over the T2 bridge), hid-apple, applesmc. QEMU has no T2 chip — those modules fail there and that is fine.

Wi‑Fi: driver is on the stick. Firmware is not (Apple’s). **[docs/wifi.md](wifi.md)** — on the 2018 in macOS run `./scripts/copy-wifi-fw.sh`, then flash. Chip **BCM4364 / Kauai**.

## Boot

1. Plug the stick into the 15,1.
2. Power on, hold **Option**.
3. Choose **EFI Boot**.
4. Hearts, then **Start**.

Black screen → unplug, boot macOS.

## Back to macOS

Unplug the stick. Power on as usual.

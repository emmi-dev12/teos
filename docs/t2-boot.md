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

## What is already on the stick (offline)

No extra download on the 15,1 for these:

- Boot film (mate hearts) + settings boxes
- Linux home (click or type)
- Kernel cmdline from the [t2linux basic setup](https://t2linux.org/guides/postinstall/): `intel_iommu=on iommu=pt pm_async=off`
- Public **apple_set_os** EFI protocol call (same idea as [0xbb/apple_set_os.efi](https://github.com/0xbb/apple_set_os.efi)) so the laptop may keep the Intel GPU awake. Harmless if the protocol is missing (QEMU).

## What still needs a t2linux kernel

Vanilla Alpine **will not** drive internal keyboard, trackpad, Touch Bar, speakers, fans, or Broadcom Wi‑Fi. That kernel is [t2linux](https://t2linux.org/) — GPL patches, not something we reverse-engineer.

Drop a t2 `vmlinuz` on the stick as `EFI/TEOS/vmlinuz` **on the build Mac** (internet there is fine). The 15,1 then boots it with no network.

Wi‑Fi firmware is Apple’s. We do **not** ship it. If you already copied `brcmfmac4364-pcie.*` off **this** MacBook (MacBookPro15,1 / Kauai) while in macOS, put them in `third_party/brcm/` on the build Mac; `make-usb.sh` copies them if present. See the [t2linux wifi guide](https://t2linux.org/guides/wifi/).

## Boot

1. Plug the stick into the 15,1.
2. Power on, hold **Option**.
3. Choose **EFI Boot**.
4. Hearts, then **Start**.

Black screen → unplug, boot macOS.

## Back to macOS

Unplug the stick. Power on as usual.

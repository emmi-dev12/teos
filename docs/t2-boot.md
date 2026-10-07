# TeOS on MacBookPro15,1 (USB only)

Read [DISCLAIMER.md](../DISCLAIMER.md) first. Dual-boot is **not** supported.

Target: **MacBook Pro 2018 15-inch**, model **MacBookPro15,1**, T2 chip. Build host may be another Mac (e.g. Apple Silicon) or Linux.

## Allow external boot (you do this, once)

1. Shut down.
2. Power on and hold **Cmd-R** until Recovery.
3. Utilities → **Startup Security Utility**.
4. Allow **external media** / reduced security as Apple’s screen describes.
5. Restart into macOS.

We do not document firmware exploits or T2 bypasses. If you cannot find that setting, stop.

## Flash a stick

On the build Mac:

```bash
diskutil list
# pick the USB. If it is disk0 you are wrong.
TEOS_DISK=diskN ./scripts/make-usb.sh
```

The script **exits if the id is disk0**.

## Boot

1. Plug the stick into the 15,1.
2. Power on, hold **Option**.
3. Choose **EFI Boot**.
4. Expect a dark screen with **TEOS** in cyan, or a black screen (then unplug and boot macOS).

Internal keyboard/trackpad/Wi‑Fi/AMD GPU are **not** supported in v0.1. An external USB keyboard may work later; v0.1 just paints and sits.

## Back to macOS

Unplug the stick. Power on as usual.

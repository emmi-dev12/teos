# Mate

TeOS is named after **mate** — the drink, the word, the person you sit with.

The default boot film is two hearts on a dark wine screen, cream **MATE** underneath. It lives in `esp/CLIP.VID`. Tap **My clip** on the boot screen (already the default).

Change it: `./scripts/add-clip.sh your.mp4` or **Upload** on this Mac. Firmware cannot open Photos.

Rebuild without a custom clip: delete `esp/CLIP.VID` and run `./scripts/build.sh` (it runs `scripts/mate-clip.py`).

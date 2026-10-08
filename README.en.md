# TeOS

> **[READ THE DISCLAIMER FIRST](DISCLAIMER.md)**  
> Experimental. **USB only.** Can destroy data. No warranty. Not affiliated with Apple.

[DE](README.md) · EN

TeOS is Linux (so apps can install) plus a **picture settings screen** at boot. You do **not** edit config files.

The name is **mate**. Default boot film: two hearts. Story: [docs/mate.md](docs/mate.md).

Top is a live preview. Then boxes: **Circles / Grow / Rain / Bounce / My clip**, colors, **Short / Medium / Long**, **Save**. Click, or arrows then Enter.

Your own video: tap **Upload** on the boot screen, or on this Mac run `./scripts/upload-gui.py` (a real file picker). Firmware cannot open Photos.

Linux: `./scripts/run-qemu.sh` then **Start** (or space). Click the boxes (or type 1–0). Or `./scripts/run-linux.sh`.

```bash
./scripts/build.sh
./scripts/run-qemu.sh          # splash, then Start
./scripts/run-linux.sh         # skip splash, home screen
```

## USB

Read **DISCLAIMER.md** and **docs/t2-boot.md**. Scripts refuse `disk0`. T2 iommu cmdline + apple_set_os are baked into the USB (offline). Internal keyboard/Wi‑Fi still need a t2linux kernel dropped on the stick at build time.

## License

MIT. The disclaimer still applies.

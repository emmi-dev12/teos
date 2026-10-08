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
./scripts/make-iso.sh          # build/teos.iso
./scripts/run-iso.sh           # QEMU from the ISO
```

GitHub Actions builds `teos.iso` on every push to `main` (Actions → ISO → artifact **teos-iso**). Do not commit the ISO.

## USB

Read **DISCLAIMER.md** and **docs/t2-boot.md**. Scripts refuse `disk0`. USB kernel is **t2linux 6.18.54** (GPL). Wi‑Fi: [docs/wifi.md](docs/wifi.md) (BCM4364 Kauai; firmware from the 15,1, not shipped).

## License

MIT. The disclaimer still applies.

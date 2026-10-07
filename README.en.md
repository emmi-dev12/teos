# TeOS

> **[READ THE DISCLAIMER FIRST](DISCLAIMER.md)**  
> Experimental. **USB only.** Can destroy data. No warranty. Not affiliated with Apple.

[DE](README.md) · EN

TeOS is Linux (so apps can install) plus a **picture settings screen** at boot. You do **not** edit config files.

Top is a live preview. Then boxes: **Circles / Grow / Rain / Bounce / My clip**, colors, **Short / Medium / Long**, **Save**. Click, or arrows then Enter.

Your own video: `./scripts/add-clip.sh ~/Desktop/your.mp4` then tap **My clip**. (The boot chip cannot open Photos — the file has to be on the stick.)

Linux apps in QEMU still use a login for now (`root`, empty password). Same-style settings there is next.

```bash
./scripts/build.sh
./scripts/run-qemu.sh          # splash
./scripts/fetch-alpine.sh
./scripts/run-linux.sh         # Alpine: login root, empty password, then apk add nano
```

## USB

Read **DISCLAIMER.md** and **docs/t2-boot.md**. Scripts refuse `disk0`. Vanilla Alpine **will not** drive T2 Wi‑Fi / internal keyboard; that needs a t2linux kernel later.

## License

MIT. The disclaimer still applies.

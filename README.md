# TeOS

> **[DISCLAIMER — ZUERST LESEN](DISCLAIMER.md)**  
> Experimentell. **Nur USB.** Kann Daten vernichten. Keine Garantie. Nicht mit Apple verbunden.

DE · [EN](README.en.md)

TeOS ist ein **eigenes Linux** (Alpine, damit `apk add` / Linux-Apps gehen) plus eigener **UEFI-Boot-Splash**. Linux-Apps brauchen einen Linux-Kernel. Ziel: **MacBook Pro 2018 15"** (`MacBookPro15,1`). Dual-Boot **gibt es nicht**. Nur USB.

Anpassen: `config/splash.cfg`, `config/linux.cfg` — [config/README.md](config/README.md).

```bash
./scripts/build.sh && ./scripts/run-qemu.sh
./scripts/fetch-alpine.sh && ./scripts/run-linux.sh
```

MIT. Haftungsausschluss in DISCLAIMER.md gilt.

# TeOS

> **[DISCLAIMER — ZUERST LESEN](DISCLAIMER.md)**  
> Experimenteller Hobby-Kernel. **Nur USB.** Nicht Linux. Nicht macOS. Kann Daten vernichten. Keine Garantie. Nicht mit Apple verbunden.

DE · [EN](README.en.md)

TeOS ist ein **eigenes** Betriebssystem (kein Linux-/Unix-Klon). Zielgerät später: **MacBook Pro 2018 15"** (`MacBookPro15,1`, T2). Heute: **QEMU** zuerst, dann USB-Stick. Dual-Boot gibt es **noch nicht**.

## Stand

v0.1 in Arbeit: UEFI-Loader `BOOTX64.EFI`, der `TeOS` auf den Bildschirm schreibt.

## Bauen

Auf macOS (Apple Silicon cross) oder Linux:

```bash
rustup target add x86_64-unknown-uefi
./scripts/build.sh
./scripts/run-qemu.sh
```

## USB (nur Stick, nie die interne Platte)

Lies **[DISCLAIMER.md](DISCLAIMER.md)** und **[docs/t2-boot.md](docs/t2-boot.md)**. Skripte verweigern `disk0`.

## Lizenz

MIT. Haftungsausschluss in DISCLAIMER.md gilt trotzdem.

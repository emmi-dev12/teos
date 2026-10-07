# TeOS

> **[READ THE DISCLAIMER FIRST](DISCLAIMER.md)**  
> Experimental. **USB only.** Can destroy data. No warranty. Not affiliated with Apple.

[DE](README.md) · EN

TeOS is a **custom Linux** (Alpine userspace so `apk add` / normal Linux apps work) plus our own **UEFI splash** (not a from-scratch kernel anymore — Linux apps need a Linux kernel). Target later: **MacBook Pro 2018 15"** (`MacBookPro15,1`). Dual-boot is **not** implemented. USB only.

## Customize

Edit text files, rebuild/copy, reboot. See [config/README.md](config/README.md).

- `config/splash.cfg` — boot animation (`orbit` / `pulse` / `rain` / `bounce` / `none`), colors, duration
- `config/linux.cfg` — hostname, extra packages, motd

## Run (this Mac, QEMU)

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

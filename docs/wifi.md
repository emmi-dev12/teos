# Wi‑Fi on MacBookPro15,1

TeOS uses the t2linux kernel, so the **driver** (`brcmfmac`) is already on the stick. Apple’s **firmware** is not. We do not download or ship those files.

Chip: **BCM4364 rev 3**, island **Kauai**. Guide: [t2linux Wi‑Fi](https://t2linux.org/guides/wifi/).

## Once, in macOS on the 2018

```bash
cd ~/teos
./scripts/copy-wifi-fw.sh
```

That reads `ioreg` / the Kauai files and writes renamed blobs into `third_party/brcm/`. Then flash:

```bash
TEOS_DISK=diskN ./scripts/make-usb.sh
```

If the script cannot find files, copy them yourself (same names):

- `brcmfmac4364-pcie.bin` (from `kauai.trx`)
- `brcmfmac4364-pcie.clm_blob` (from `kauai-X3.clmb`)
- `brcmfmac4364-pcie.Apple Inc.-MacBookPro15,1.txt` (from the NVRAM `.txt`)

Typical macOS sources: `/usr/share/firmware/wifi/C-4364s-B2/`. Confirm with:

```bash
ioreg -l | grep RequestedFiles
sysctl -n hw.model   # must be MacBookPro15,1
```

## On TeOS

Init copies `EFI/TEOS/brcm/` from the stick to `/lib/firmware/brcm/` and loads `brcmfmac`. No extra internet step.

This M4 build Mac has no Kauai chip — run `copy-wifi-fw.sh` on the **15,1**.

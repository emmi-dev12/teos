#!/bin/sh
set -eu
cd "$(dirname "$0")/.."
rustup target add x86_64-unknown-uefi >/dev/null
cargo build -p teos-boot --target x86_64-unknown-uefi --release
mkdir -p esp/EFI/BOOT esp/EFI/TEOS
cp target/x86_64-unknown-uefi/release/teos-boot.efi esp/EFI/BOOT/BOOTX64.EFI
cp config/splash.cfg esp/splash.cfg
cp config/splash.cfg esp/EFI/TEOS/splash.cfg
if [ ! -f esp/CLIP.VID ]; then
  python3 - <<'PY'
from pathlib import Path
import struct, math
w, h, n = 160, 90, 24
frames = bytearray()
for i in range(n):
    cx = int(40 + 80 * (0.5 + 0.5 * math.sin(i / n * 6.28)))
    cy = h // 2
    for y in range(h):
        for x in range(w):
            d = (x - cx) ** 2 + (y - cy) ** 2
            on = d < 180
            frames += bytes([0x20 if on else 12, 0xC8 if on else 12, 0xFF if on else 16])
Path("esp/CLIP.VID").write_bytes(b"TEOSCLIP" + struct.pack("<HHH", w, h, n) + frames)
print("demo CLIP.VID")
PY
fi
echo "wrote esp/EFI/BOOT/BOOTX64.EFI + splash.cfg"

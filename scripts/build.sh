#!/bin/sh
set -eu
cd "$(dirname "$0")/.."
rustup target add x86_64-unknown-uefi >/dev/null
cargo build -p teos-boot --target x86_64-unknown-uefi --release
mkdir -p esp/EFI/BOOT esp/EFI/TEOS
cp target/x86_64-unknown-uefi/release/teos-boot.efi esp/EFI/BOOT/BOOTX64.EFI
cp config/splash.cfg esp/splash.cfg
cp config/splash.cfg esp/EFI/TEOS/splash.cfg
# Mate clip is the default boot film. Keep a user clip if they already uploaded one.
if [ ! -f esp/CLIP.VID ]; then
  python3 scripts/mate-clip.py
else
  sz=$(wc -c < esp/CLIP.VID)
  if [ "$sz" -eq 1036814 ] || [ "$sz" -eq 129614 ]; then
    python3 scripts/mate-clip.py
  fi
fi
echo "wrote esp/EFI/BOOT/BOOTX64.EFI + splash.cfg"

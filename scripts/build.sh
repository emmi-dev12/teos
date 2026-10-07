#!/bin/sh
set -eu
cd "$(dirname "$0")/.."
rustup target add x86_64-unknown-uefi >/dev/null
cargo build -p teos-boot --target x86_64-unknown-uefi --release
mkdir -p esp/EFI/BOOT esp/EFI/TEOS
cp target/x86_64-unknown-uefi/release/teos-boot.efi esp/EFI/BOOT/BOOTX64.EFI
cp config/splash.cfg esp/splash.cfg
cp config/splash.cfg esp/EFI/TEOS/splash.cfg
echo "wrote esp/EFI/BOOT/BOOTX64.EFI + splash.cfg"

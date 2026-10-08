#!/bin/sh
# UEFI ISO: ISO9660 files + FAT ESP as GPT partition 2 (hybrid).
# QEMU: ./scripts/run-iso.sh
# USB (not disk0): dd if=build/teos.iso of=/dev/rdiskN bs=4m
set -eu
cd "$(dirname "$0")/.."
./scripts/build.sh
if [ ! -f esp/EFI/TEOS/vmlinuz ] || [ ! -f esp/EFI/TEOS/initrd.gz ]; then
  ./scripts/pack-linux.sh
fi

rm -rf build/iso
mkdir -p build/iso/EFI/BOOT build/iso/EFI/TEOS
cp -f esp/EFI/BOOT/BOOTX64.EFI build/iso/EFI/BOOT/
cp -f esp/EFI/TEOS/* build/iso/EFI/TEOS/ 2>/dev/null || true
cp -f esp/splash.cfg build/iso/splash.cfg 2>/dev/null || true
cp -f esp/CLIP.VID build/iso/CLIP.VID 2>/dev/null || true
cp -f DISCLAIMER.md build/iso/DISCLAIMER.md
cp -f docs/t2-boot.md build/iso/T2.TXT 2>/dev/null || true
cp -f docs/mate.md build/iso/MATE.TXT 2>/dev/null || true
cp -f docs/wifi.md build/iso/WIFI.TXT 2>/dev/null || true
printf '%s\n' '\EFI\BOOT\BOOTX64.EFI' > build/iso/startup.nsh
printf '%s\n' '\EFI\BOOT\BOOTX64.EFI' > build/iso/EFI/BOOT/startup.nsh

sz=$(du -sk build/iso | awk '{print $1}')
need=$((sz + 16384))
if [ "$need" -lt 40960 ]; then need=40960; fi
EFIMG=build/efiboot.img
rm -f "$EFIMG"
dd if=/dev/zero of="$EFIMG" bs=1024 count="$need" status=none
mkfs.vfat -F 32 -n TEOS "$EFIMG"
mmd -i "$EFIMG" ::EFI ::EFI/BOOT ::EFI/TEOS
mcopy -i "$EFIMG" build/iso/EFI/BOOT/BOOTX64.EFI ::EFI/BOOT/
for f in build/iso/EFI/TEOS/*; do
  [ -f "$f" ] && mcopy -i "$EFIMG" "$f" ::EFI/TEOS/
done
mcopy -i "$EFIMG" build/iso/splash.cfg ::splash.cfg 2>/dev/null || true
mcopy -i "$EFIMG" build/iso/CLIP.VID ::CLIP.VID 2>/dev/null || true
mcopy -i "$EFIMG" build/iso/DISCLAIMER.md ::DISCLAIMER.md
mcopy -i "$EFIMG" build/iso/startup.nsh ::startup.nsh

rm -f build/teos.iso
xorriso -as mkisofs \
  -V TEOS \
  -iso-level 3 \
  -R -J \
  -append_partition 2 0xef "$EFIMG" \
  -e --interval:appended_partition_2:all:: \
  -no-emul-boot \
  -isohybrid-gpt-basdat \
  -o build/teos.iso \
  build/iso

echo "ISO $(wc -c < build/teos.iso) bytes -> build/teos.iso"
file build/teos.iso
echo "QEMU: ./scripts/run-iso.sh"
echo "USB (not disk0): dd if=build/teos.iso of=/dev/rdiskN bs=4m"

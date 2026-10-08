#!/bin/sh
# Copy TeOS EFI to a USB stick. NEVER the internal disk.
set -eu
cd "$(dirname "$0")/.."

echo "Read DISCLAIMER.md. USB only. Internal SSD will be refused."
if [ -z "${TEOS_DISK:-}" ]; then
  echo "Set TEOS_DISK to the stick identifier (example: disk4), not disk0." >&2
  echo "List: diskutil list" >&2
  exit 2
fi

case "$TEOS_DISK" in
  disk0|disk0s*|rdisk0|rdisk0s*)
    echo "REFUSED: $TEOS_DISK looks like the internal disk." >&2
    exit 3
    ;;
  disk[0-9]*|rdisk[0-9]*)
    ;;
  *)
    echo "REFUSED: TEOS_DISK='$TEOS_DISK' is not a diskN id." >&2
    exit 3
    ;;
esac

# Extra belt: whole-disk size heuristic is not enough; still block disk0 only.
DEV="/dev/$TEOS_DISK"
if [ ! -e "$DEV" ]; then
  echo "No $DEV" >&2
  exit 4
fi

./scripts/build.sh
./scripts/pack-linux.sh
echo "About to erase $DEV as FAT32 TEOS. Ctrl-C if wrong."
sleep 5
diskutil eraseDisk FAT32 TEOS MBR "$DEV"
mkdir -p /Volumes/TEOS/EFI/BOOT /Volumes/TEOS/EFI/TEOS
cp esp/EFI/BOOT/BOOTX64.EFI /Volumes/TEOS/EFI/BOOT/BOOTX64.EFI
cp -R esp/EFI/TEOS/. /Volumes/TEOS/EFI/TEOS/
cp -f esp/splash.cfg /Volumes/TEOS/splash.cfg 2>/dev/null || true
cp -f esp/CLIP.VID /Volumes/TEOS/CLIP.VID 2>/dev/null || true
cp DISCLAIMER.md /Volumes/TEOS/DISCLAIMER.md
cp docs/t2-boot.md /Volumes/TEOS/T2.TXT 2>/dev/null || true
cp docs/mate.md /Volumes/TEOS/MATE.TXT 2>/dev/null || true
if [ -d third_party/brcm ]; then
  mkdir -p /Volumes/TEOS/EFI/TEOS/brcm
  cp -f third_party/brcm/* /Volumes/TEOS/EFI/TEOS/brcm/ 2>/dev/null || true
fi
diskutil eject "$DEV" || true
echo "Stick ready. Hold Option on the T2 Mac, pick EFI Boot, then Start."

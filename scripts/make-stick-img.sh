#!/bin/sh
# QEMU user disk (real FAT32) + ext4 APPS.IMG (unix files; vfat cannot hold apk).
set -eu
cd "$(dirname "$0")/.."
mkdir -p build
PATH="/opt/homebrew/opt/e2fsprogs/sbin:/opt/homebrew/bin:$PATH"
IMG=build/stick.img
APPS=build/apps.img
sz=0
if [ -f "$IMG" ]; then
  sz=$(wc -c < "$IMG")
fi
if [ ! -f "$IMG" ] || [ "$sz" -lt 200000000 ]; then
  dd if=/dev/zero of="$IMG" bs=1048576 count=256 status=none
  mkfs.vfat -F 32 -n TEOS "$IMG"
  echo "created $IMG"
else
  echo "using $IMG"
fi
if [ ! -f "$APPS" ]; then
  dd if=/dev/zero of="$APPS" bs=1048576 count=96 status=none
  mkfs.ext4 -F -L TEOSAPPS "$APPS"
  echo "created $APPS"
fi
# Put APPS.IMG on the FAT stick if missing.
if command -v mdir >/dev/null 2>&1 && mdir -i "$IMG" ::APPS.IMG >/dev/null 2>&1; then
  echo "APPS.IMG already on stick"
  exit 0
fi
if command -v mcopy >/dev/null 2>&1; then
  mcopy -o -i "$IMG" "$APPS" ::APPS.IMG
  echo "copied APPS.IMG onto stick"
  exit 0
fi
OUT=$(hdiutil attach -imagekey diskimage-class=CRawDiskImage -nobrowse "$IMG")
VOL=$(printf '%s\n' "$OUT" | awk '/\/Volumes\// {print $NF; exit}')
if [ -z "$VOL" ] || [ ! -d "$VOL" ]; then
  echo "could not mount $IMG to copy APPS.IMG" >&2
  echo "$OUT" >&2
  exit 1
fi
cp -f "$APPS" "$VOL/APPS.IMG"
sync
hdiutil detach "$VOL" >/dev/null
echo "copied APPS.IMG via hdiutil"

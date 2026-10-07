#!/bin/sh
# QEMU user disk (real FAT32). vvfat cannot remount in Linux.
set -eu
cd "$(dirname "$0")/.."
mkdir -p build
IMG=build/stick.img
if [ ! -f "$IMG" ]; then
  dd if=/dev/zero of="$IMG" bs=1048576 count=64 status=none
  mkfs.vfat -F 32 -n TEOS "$IMG"
  echo "created $IMG"
else
  echo "using $IMG"
fi

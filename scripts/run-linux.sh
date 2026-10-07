#!/bin/sh
set -eu
cd "$(dirname "$0")/.."
./scripts/pack-linux.sh
DISP="-display cocoa"
if [ "$(uname -s)" != "Darwin" ]; then DISP="-display gtk"; fi
if [ "${TEOS_HEADLESS:-}" = 1 ]; then DISP="-display none"; fi
echo "TeOS Linux — no login. Type 1-4."
exec qemu-system-x86_64 \
  -machine q35 \
  -m 512 \
  -kernel build/vmlinuz-virt \
  -initrd build/teos-initrd.gz \
  -append "console=ttyS0 rdinit=/init quiet" \
  -nic user,model=virtio \
  -serial stdio \
  $DISP

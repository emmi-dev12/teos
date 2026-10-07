#!/bin/sh
# Boot Alpine Linux (TeOS userspace) in QEMU. apk add works with user networking.
set -eu
cd "$(dirname "$0")/.."
ISO="build/alpine-virt-3.24.2-x86_64.iso"
if [ ! -f "$ISO" ]; then
  echo "missing $ISO — run scripts/fetch-alpine.sh" >&2
  exit 1
fi

DISP="-display cocoa"
if [ "$(uname -s)" != "Darwin" ]; then DISP="-display gtk"; fi
if [ "${TEOS_HEADLESS:-}" = 1 ]; then DISP="-display none"; fi

echo "Alpine login: root  (empty password)"
echo "Then: apk update && apk add nano"
echo "USB-only on real Mac. Read DISCLAIMER.md"

exec qemu-system-x86_64 \
  -machine q35 \
  -m 1024 \
  -cdrom "$ISO" \
  -nic user,model=virtio \
  -serial stdio \
  $DISP

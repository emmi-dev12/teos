#!/bin/sh
set -eu
cd "$(dirname "$0")/.."
./scripts/pack-linux.sh
./scripts/make-stick-img.sh
DISP="-display cocoa"
if [ "$(uname -s)" != "Darwin" ]; then DISP="-display gtk"; fi
if [ "${TEOS_HEADLESS:-}" = 1 ]; then DISP="-display none"; fi
KERN=build/vmlinuz-virt
if [ -f build/vmlinuz-t2 ]; then KERN=build/vmlinuz-t2; fi
echo "TeOS Linux — click a box, or type a number. kernel=$KERN"
exec qemu-system-x86_64 \
  -machine q35 \
  -m 384 \
  -kernel "$KERN" \
  -initrd build/teos-initrd.gz \
  -append "console=ttyS0 rdinit=/init quiet" \
  -vga std \
  -device qemu-xhci,id=xhci \
  -device usb-tablet,bus=xhci.0 \
  -drive format=raw,file=fat:rw:esp \
  -drive file=build/stick.img,format=raw,if=virtio \
  -nic user,model=virtio \
  -serial stdio \
  $DISP

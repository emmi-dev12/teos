#!/bin/sh
set -eu
cd "$(dirname "$0")/.."
mkdir -p build linux-root
ISO="build/alpine-virt-3.24.2-x86_64.iso"
ROOTFS="build/alpine-minirootfs-3.24.2-x86_64.tar.gz"
if [ ! -f "$ISO" ]; then
  ./scripts/fetch-alpine.sh
fi
if [ ! -f "$ROOTFS" ]; then
  curl -L --fail --retry 3 -o "$ROOTFS" \
    https://dl-cdn.alpinelinux.org/alpine/v3.24/releases/x86_64/alpine-minirootfs-3.24.2-x86_64.tar.gz
fi
if [ ! -f build/vmlinuz-virt ]; then
  7z e -y -obuild "$ISO" boot/vmlinuz-virt >/dev/null
fi

rm -rf linux-root
mkdir linux-root
tar -C linux-root -xzf "$ROOTFS"
mkdir -p linux-root/usr/local/bin
cp linux/teos-ui.sh linux-root/usr/local/bin/teos-ui
cp linux/init linux-root/init
chmod 755 linux-root/init linux-root/usr/local/bin/teos-ui
printf 'teos\n' > linux-root/etc/hostname
printf 'teos\n' > linux-root/etc/motd

# newc cpio (macOS cpio supports -H newc)
(
  cd linux-root
  find . | cpio -o -H newc 2>/dev/null
) | gzip -9 > build/teos-initrd.gz
mkdir -p esp/EFI/TEOS
cp -f build/vmlinuz-virt esp/EFI/TEOS/vmlinuz
cp -f build/teos-initrd.gz esp/EFI/TEOS/initrd.gz
echo "packed build/teos-initrd.gz ($(wc -c < build/teos-initrd.gz) bytes)"

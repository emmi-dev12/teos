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
if command -v zig >/dev/null 2>&1; then
  zig cc -target x86_64-linux-musl -static -O2 -s -o linux-root/usr/local/bin/teos-gui linux/teos-gui.c
elif [ -x linux/teos-gui ]; then
  cp linux/teos-gui linux-root/usr/local/bin/teos-gui
fi
chmod 755 linux-root/usr/local/bin/teos-ui linux-root/usr/local/bin/teos-gui 2>/dev/null || true
cp linux/init linux-root/init
chmod 755 linux-root/init linux-root/usr/local/bin/teos-ui
printf 'teos\n' > linux-root/etc/hostname
printf 'teos\n' > linux-root/etc/motd
printf '%s\n' \
  'http://dl-cdn.alpinelinux.org/alpine/v3.24/main' \
  'http://dl-cdn.alpinelinux.org/alpine/v3.24/community' \
  > linux-root/etc/apk/repositories

if [ ! -f build/modloop-virt ]; then
  7z e -y -obuild "$ISO" boot/modloop-virt >/dev/null
fi
KVER=6.18.52-0-virt
mkdir -p build/mods
unsquashfs -f -d build/mods build/modloop-virt \
    modules/$KVER/kernel/drivers/scsi/sd_mod.ko \
    modules/$KVER/kernel/fs/nls/nls_cp437.ko \
    modules/$KVER/kernel/fs/nls/nls_iso8859-1.ko \
    modules/$KVER/kernel/fs/fat/fat.ko \
    modules/$KVER/kernel/fs/fat/vfat.ko \
    modules/$KVER/kernel/net/core/failover.ko \
    modules/$KVER/kernel/drivers/net/net_failover.ko \
    modules/$KVER/kernel/drivers/net/virtio_net.ko \
    modules/$KVER/kernel/drivers/net/ethernet/intel/e1000/e1000.ko \
    modules/$KVER/kernel/net/packet/af_packet.ko \
    modules/$KVER/kernel/drivers/block/virtio_blk.ko \
    modules/$KVER/kernel/drivers/usb/common/usb-common.ko \
    modules/$KVER/kernel/drivers/usb/core/usbcore.ko \
    modules/$KVER/kernel/drivers/usb/host/xhci-hcd.ko \
    modules/$KVER/kernel/drivers/usb/host/xhci-pci.ko \
    modules/$KVER/kernel/drivers/usb/storage/usb-storage.ko \
    modules/$KVER/kernel/drivers/usb/storage/uas.ko \
    modules/$KVER/kernel/drivers/block/loop.ko \
    modules/$KVER/kernel/lib/crc/crc16.ko \
    modules/$KVER/kernel/fs/mbcache.ko \
    modules/$KVER/kernel/fs/jbd2/jbd2.ko \
    modules/$KVER/kernel/fs/ext4/ext4.ko \
    modules/$KVER/kernel/drivers/i2c/i2c-core.ko \
    modules/$KVER/kernel/drivers/video/fbdev/core/fb.ko \
    modules/$KVER/kernel/drivers/video/fbdev/core/fb_sys_fops.ko \
    modules/$KVER/kernel/drivers/video/fbdev/core/syscopyarea.ko \
    modules/$KVER/kernel/drivers/video/fbdev/core/sysfillrect.ko \
    modules/$KVER/kernel/drivers/video/fbdev/core/sysimgblt.ko \
    modules/$KVER/kernel/drivers/gpu/drm/drm_panel_orientation_quirks.ko \
    modules/$KVER/kernel/drivers/gpu/drm/drm.ko \
    modules/$KVER/kernel/drivers/gpu/drm/drm_kms_helper.ko \
    modules/$KVER/kernel/drivers/gpu/drm/clients/drm_client_lib.ko \
    modules/$KVER/kernel/drivers/gpu/drm/drm_shmem_helper.ko \
    modules/$KVER/kernel/drivers/gpu/drm/sysfb/drm_sysfb_helper.ko \
    modules/$KVER/kernel/drivers/gpu/drm/sysfb/simpledrm.ko \
    modules/$KVER/kernel/drivers/gpu/drm/ttm/ttm.ko \
    modules/$KVER/kernel/drivers/gpu/drm/drm_ttm_helper.ko \
    modules/$KVER/kernel/drivers/gpu/drm/drm_vram_helper.ko \
    modules/$KVER/kernel/drivers/gpu/drm/tiny/bochs.ko \
    modules/$KVER/kernel/drivers/hid/hid.ko \
    modules/$KVER/kernel/drivers/hid/hid-generic.ko \
    modules/$KVER/kernel/drivers/hid/usbhid/usbhid.ko \
    modules/$KVER/kernel/drivers/input/evdev.ko \
    modules/$KVER/kernel/drivers/input/mousedev.ko \
    >/dev/null
if [ -d build/mods/modules/$KVER ]; then
  mkdir -p linux-root/lib/modules/$KVER
  cp -R build/mods/modules/$KVER/kernel linux-root/lib/modules/$KVER/
fi

./scripts/fetch-t2-kernel.sh
if [ -d build/t2deb/extract/lib/modules ]; then
  mkdir -p linux-root/lib/modules
  cp -R build/t2deb/extract/lib/modules/. linux-root/lib/modules/
fi

# Drop weight: docs, man, unused share. Keep apk + ssl.
rm -rf linux-root/usr/share/man linux-root/usr/share/doc \
  linux-root/usr/share/misc linux-root/usr/share/aclocal \
  linux-root/media linux-root/mnt linux-root/opt linux-root/srv \
  linux-root/home linux-root/usr/local/share 2>/dev/null || true
find linux-root -name '*.a' -delete 2>/dev/null || true

# zstd: smaller than gzip-1, much faster to unpack than gzip-9 on old CPUs.
# Kernel sniffs magic; filename stays initrd.gz for the EFI stub.
(
  cd linux-root
  find . | cpio -o -H newc 2>/dev/null
) | zstd -1 -T0 -o build/teos-initrd.gz --force
mkdir -p esp/EFI/TEOS
if [ -f build/vmlinuz-t2 ]; then
  cp -f build/vmlinuz-t2 esp/EFI/TEOS/vmlinuz
  echo "kernel t2linux $(file -b build/vmlinuz-t2 | cut -c1-80)"
else
  cp -f build/vmlinuz-virt esp/EFI/TEOS/vmlinuz
fi
cp -f build/teos-initrd.gz esp/EFI/TEOS/initrd.gz
echo "packed build/teos-initrd.gz ($(wc -c < build/teos-initrd.gz) bytes)"

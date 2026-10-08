#!/bin/sh
# Official t2linux kernel (GPL). Not a T2 dump. Build Mac only.
# https://github.com/t2linux/T2-Debian-and-Ubuntu-Kernel
set -eu
cd "$(dirname "$0")/.."
mkdir -p build
VER=6.18.54-1
DEB="linux-image-${VER}-t2-bookworm_${VER}_amd64.deb"
URL="https://github.com/t2linux/T2-Debian-and-Ubuntu-Kernel/releases/download/v${VER}/$DEB"
if [ -f build/vmlinuz-t2 ]; then
  echo "have build/vmlinuz-t2"
  exit 0
fi
if [ ! -f "build/$DEB" ]; then
  curl -L --fail --retry 3 -o "build/$DEB" "$URL"
fi
rm -rf build/t2deb
mkdir -p build/t2deb
(
  cd build/t2deb
  ar x "../$DEB"
  mkdir -p extract
  tar -xJf data.tar.xz -C extract ./boot "./lib/modules/${VER}-t2-bookworm/kernel"
)
cp "build/t2deb/extract/boot/vmlinuz-${VER}-t2-bookworm" build/vmlinuz-t2
find build/t2deb/extract/lib/modules -name '*.ko.xz' -print0 | xargs -0 unxz -f
echo "t2linux kernel $VER -> build/vmlinuz-t2"
file build/vmlinuz-t2

#!/bin/sh
set -eu
cd "$(dirname "$0")/.."
./scripts/build.sh
./scripts/pack-linux.sh
./scripts/make-stick-img.sh

CODE=""
VARS_SRC=""
for p in \
  /opt/homebrew/share/qemu/edk2-x86_64-code.fd \
  /usr/share/OVMF/OVMF_CODE.fd \
  /usr/share/edk2/x64/OVMF_CODE.fd \
  /usr/share/qemu/edk2-x86_64-code.fd
do
  if [ -f "$p" ]; then CODE="$p"; break; fi
done
for p in \
  /opt/homebrew/share/qemu/edk2-i386-vars.fd \
  /usr/share/OVMF/OVMF_VARS.fd \
  /usr/share/edk2/x64/OVMF_VARS.fd
do
  if [ -f "$p" ]; then VARS_SRC="$p"; break; fi
done
if [ -z "$CODE" ]; then
  echo "OVMF/edk2 firmware not found. Install qemu (brew install qemu)." >&2
  exit 1
fi

mkdir -p build
VARS="build/ovmf_vars.fd"
if [ -n "$VARS_SRC" ] && [ ! -f "$VARS" ]; then
  cp "$VARS_SRC" "$VARS"
fi

DISP="-display cocoa"
if [ "$(uname -s)" != "Darwin" ]; then DISP="-display gtk"; fi
if [ "${TEOS_HEADLESS:-}" = 1 ]; then DISP="-display none"; fi

FLASH_VARS=""
if [ -f "$VARS" ]; then
  FLASH_VARS="-drive if=pflash,format=raw,file=$VARS"
fi

exec qemu-system-x86_64 \
  -machine q35 \
  -m 256 \
  -drive if=pflash,format=raw,readonly=on,file="$CODE" \
  $FLASH_VARS \
  -drive format=raw,file=fat:rw:esp \
  -drive file=build/stick.img,format=raw,if=virtio \
  -nic user,model=virtio \
  -serial stdio \
  $DISP

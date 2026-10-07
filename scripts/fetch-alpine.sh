#!/bin/sh
set -eu
cd "$(dirname "$0")/.."
mkdir -p build
ISO="build/alpine-virt-3.24.2-x86_64.iso"
URL="https://dl-cdn.alpinelinux.org/alpine/v3.24/releases/x86_64/alpine-virt-3.24.2-x86_64.iso"
SUM="3ab424762af704b2c2a9e57df1dc37f982af260071504d977f2fb96822e7130b"
if [ ! -f "$ISO" ]; then
  curl -L --fail --retry 3 -o "$ISO" "$URL"
fi
got=$(shasum -a 256 "$ISO" | awk '{print $1}')
if [ "$got" != "$SUM" ]; then
  echo "sha256 mismatch: $got" >&2
  exit 1
fi
echo "ok $ISO"

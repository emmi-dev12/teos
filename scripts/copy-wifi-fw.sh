#!/bin/sh
# Copy Apple Wi-Fi firmware from THIS Mac into third_party/brcm/.
# Run in macOS on MacBookPro15,1. We do not download blobs.
set -eu
cd "$(dirname "$0")/.."
OUT=third_party/brcm
mkdir -p "$OUT"

MODEL=$(sysctl -n hw.model 2>/dev/null || echo unknown)
echo "model: $MODEL"
if [ "$MODEL" != "MacBookPro15,1" ]; then
  echo "Run this on the 2018 15-inch while in macOS. This Mac is $MODEL."
  echo "NVRAM is board-specific. Do not copy firmware from another Mac."
  exit 2
fi

# Known Kauai names from t2linux (BCM4364 / MacBookPro15,1).
search() {
  find /usr/share/firmware /usr/libexec /System/Library/Extensions /usr/local/share \
    -iname "$1" 2>/dev/null | head -1
}

TRX=$(search 'kauai.trx')
CLMB=$(search 'kauai*.clmb')
NVRAM=$(search 'P-kauai*.txt')
[ -z "$NVRAM" ] && NVRAM=$(search '*kauai*.txt')

if [ -z "$TRX" ] || [ -z "$CLMB" ] || [ -z "$NVRAM" ]; then
  echo "Could not find Kauai firmware on this Mac."
  echo "In Terminal on the 15,1:  ioreg -l | grep RequestedFiles"
  echo "Then put the three renamed files in $OUT — see docs/wifi.md"
  exit 1
fi

cp -f "$TRX" "$OUT/brcmfmac4364-pcie.bin"
cp -f "$CLMB" "$OUT/brcmfmac4364-pcie.clm_blob"
cp -f "$NVRAM" "$OUT/brcmfmac4364-pcie.Apple Inc.-MacBookPro15,1.txt"
echo "wrote:"
ls -l "$OUT"/brcmfmac4364-pcie*
echo "Next: TEOS_DISK=diskN ./scripts/make-usb.sh"

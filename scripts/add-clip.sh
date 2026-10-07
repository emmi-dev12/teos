#!/bin/sh
# Put your video on the TeOS stick as the boot clip.
# Usage: ./scripts/add-clip.sh /path/to/video.mp4
set -eu
cd "$(dirname "$0")/.."
if [ "${1:-}" = "" ]; then
  echo "Drop a video on this command:  ./scripts/add-clip.sh ~/Desktop/your.mp4"
  exit 2
fi
SRC="$1"
if [ ! -f "$SRC" ]; then
  echo "Can't find $SRC"
  exit 3
fi
mkdir -p build esp
ffmpeg -y -i "$SRC" \
  -vf "scale=320:180:force_original_aspect_ratio=decrease,pad=320:180:(ow-iw)/2:(oh-ih)/2:color=black,fps=12" \
  -frames:v 48 -f rawvideo -pix_fmt rgb24 build/clip.raw
python3 - <<'PY'
from pathlib import Path
import struct
raw = Path("build/clip.raw").read_bytes()
w, h = 320, 180
n = len(raw) // (w * h * 3)
raw = raw[: n * w * h * 3]
Path("esp/CLIP.VID").write_bytes(b"TEOSCLIP" + struct.pack("<HHH", w, h, n) + raw)
print(f"clip ready: {n} frames — tap MY CLIP on the boot screen")
PY

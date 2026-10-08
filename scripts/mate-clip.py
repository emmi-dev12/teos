#!/usr/bin/env python3
"""Mate boot clip — two hearts, because that's the name."""
from pathlib import Path
import math
import struct

W, H, N = 128, 72, 24


def heart(px, py, s):
    # classic heart implicit curve, scaled
    x = (px) / s
    y = -(py) / s
    a = x * x + y * y - 1.0
    return a * a * a - x * x * y * y * y < 0.0


def pix(frames, i, x, y, r, g, b):
    o = (i * H + y) * W * 3 + x * 3
    frames[o] = r
    frames[o + 1] = g
    frames[o + 2] = b


def main():
    frames = bytearray(W * H * 3 * N)
    for i in range(N):
        t = i / N * math.tau
        # slow orbit
        cx1 = W * 0.38 + 18 * math.cos(t)
        cy1 = H * 0.48 + 8 * math.sin(t)
        cx2 = W * 0.62 + 18 * math.cos(t + math.pi)
        cy2 = H * 0.52 + 8 * math.sin(t + math.pi)
        pulse = 10.5 + 1.4 * math.sin(t * 2)
        for y in range(H):
            for x in range(W):
                # dark wine bg
                r, g, b = 18, 8, 12
                if heart(x - cx1, y - cy1, pulse):
                    r, g, b = 210, 46, 78
                elif heart(x - cx2, y - cy2, pulse * 0.92):
                    r, g, b = 232, 120, 64
                # soft glow
                d1 = (x - cx1) ** 2 + (y - cy1) ** 2
                if d1 < 420 and r < 40:
                    r, g, b = 48, 16, 24
                pix(frames, i, x, y, r, g, b)
        # MATE — 3x5 caps, cream
        font = {
            "M": ["101", "111", "101", "101", "101"],
            "A": ["010", "101", "111", "101", "101"],
            "T": ["111", "010", "010", "010", "010"],
            "E": ["111", "100", "110", "100", "111"],
        }
        word = "MATE"
        x0 = W // 2 - 14
        y1 = H - 10
        for li, ch in enumerate(word):
            rows = font[ch]
            for ry, row in enumerate(rows):
                for rx, bit in enumerate(row):
                    if bit == "1":
                        pix(frames, i, x0 + li * 7 + rx * 2, y1 + ry, 245, 230, 210)
                        pix(frames, i, x0 + li * 7 + rx * 2 + 1, y1 + ry, 245, 230, 210)
    out = Path("esp/CLIP.VID")
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_bytes(b"TEOSCLIP" + struct.pack("<HHH", W, H, N) + frames)
    print(f"mate clip {W}x{H}x{N} -> {out} ({out.stat().st_size} bytes)")


if __name__ == "__main__":
    main()

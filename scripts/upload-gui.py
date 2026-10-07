#!/usr/bin/env python3
"""Pick a video on this Mac and put it on the TeOS stick as the boot clip."""
import subprocess
import sys
import tkinter as tk
from pathlib import Path
from tkinter import filedialog, messagebox

root = Path(__file__).resolve().parents[1]
script = root / "scripts" / "add-clip.sh"

ui = tk.Tk()
ui.withdraw()
path = filedialog.askopenfilename(
    title="Pick a video for TeOS",
    filetypes=[
        ("Video", "*.mp4 *.mov *.m4v *.webm *.mkv *.avi"),
        ("All files", "*.*"),
    ],
)
if not path:
    sys.exit(0)
try:
    subprocess.check_call(["/bin/sh", str(script), path])
except subprocess.CalledProcessError:
    messagebox.showerror("TeOS", "Could not pack that video.")
    sys.exit(1)
messagebox.showinfo("TeOS", "Done. Boot TeOS, tap Upload or My clip, then Save.")

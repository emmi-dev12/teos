# STOP. READ THIS BEFORE YOU TOUCH A USB STICK OR A MAC.

**TeOS is an unfinished hobby OS. It is not something you can live on. It is not macOS, not Windows, not a recovery tool. Userspace is Linux (Alpine) so packages can install; that does not make it safe or complete.**

If you ignore this file you can **destroy data**, **brick firmware settings**, or **leave a Mac that will not boot** until you know how to use Apple Recovery. That is on you.

## Do not

- Do **not** run TeOS as your only OS.
- Do **not** write TeOS to the internal SSD (`/dev/disk0`, Macintosh HD, APFS containers, T2-encrypted volumes).
- Do **not** use this as a dual-boot installer. Dual-boot is **not implemented**. “Later” is not “now”.
- Do **not** disable T2 security, FileVault, or firmware passwords because a README on the internet sounded cool.
- Do **not** flash random `.img` files onto a stick without checking the disk identifier **twice**.
- Do **not** expect Wi‑Fi, the internal keyboard, the trackpad, the Touch Bar, audio, the AMD GPU, sleep, or batteries to work.
- Do **not** assume QEMU success means your MacBookPro15,1 will boot it.
- Do **not** file a warranty claim with Apple and blame TeOS. Apple did not make this. We are not affiliated with Apple Inc.

## USB only (current policy)

The only supported way to try TeOS is:

1. A **throwaway USB stick** you are willing to erase.
2. The MacBook Pro **2018 15"** (`MacBookPro15,1`) with **external boot allowed** in Startup Security Utility (you change that in Recovery; you can set it back).
3. Boot picker: hold **Option** at chime, pick **EFI Boot**.
4. When you are done, **unplug the stick** and boot macOS as usual.

If the screen stays black: unplug, hold power, boot macOS. That is the expected failure mode while this is young.

## Legal

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, INCLUDING DATA LOSS, HARDWARE DAMAGE, OR A MAC THAT WILL NOT BOOT.

By cloning, building, flashing, or booting TeOS you accept that **you can lose everything on the target machine** and that **no one will pay for that**.

If you do not fully understand the above: **stop. Use macOS.**

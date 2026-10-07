# Customization

Everything user-facing is a text file. No recompile needed for splash/linux prefs.

| File | When | What |
|---|---|---|
| `config/splash.cfg` | UEFI boot | animation, colors, scale, duration |
| `config/linux.cfg` | Alpine userspace | hostname, extra `apk` packages, motd |
| `config/packages` | Alpine | extra packages, one per line (optional) |

Splash animations: `none`, `pulse`, `orbit`, `rain`, `bounce`.

Copy `splash.cfg` onto the ESP as `splash.cfg` (build.sh does this).

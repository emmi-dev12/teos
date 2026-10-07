#!/bin/sh
# TeOS home screen. Numbers, not config files.
. /etc/profile 2>/dev/null || true
export PATH="/apps/usr/bin:/apps/bin:/sbin:/usr/sbin:/bin:/usr/bin"
export LD_LIBRARY_PATH="/apps/usr/lib:/apps/lib${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"

say() { printf '%s\n' "$*"; }

banner() {
  printf '\033[2J\033[H'
  printf '\033[1;37;44m'
  printf '  TEOS  %s\n' "$(date '+%Y-%m-%d %H:%M' 2>/dev/null)"
  printf '\033[0m\n'
}

wait_key() {
  printf '\n  Press Enter... '
  read -r _ || true
}

note_path() {
  if [ -d /stick ] && [ -w /stick ]; then
    printf '%s' /stick/NOTES.TXT
  else
    printf '%s' /root/notes.txt
  fi
}

set_name() {
  banner
  say "  What should this computer be called?"
  printf '\n  Name: '
  read -r n || return
  n=$(printf '%s' "$n" | tr -cd 'A-Za-z0-9-' | cut -c1-32)
  [ -z "$n" ] && return
  printf '%s\n' "$n" >/etc/hostname
  hostname "$n" 2>/dev/null || true
  printf '%s\n' "$n" >/stick/NAME.TXT 2>/dev/null || printf '%s\n' "$n" >/stick/hostname 2>/dev/null || true
  sync 2>/dev/null || true
  say ""
  say "  OK. It is called $n"
  wait_key
}

get_apps() {
  banner
  say "  Get apps (needs internet)"
  say ""
  say "  1) Text editor (nano)"
  say "  2) Web (lynx)"
  say "  3) System monitor (htop)"
  say "  4) Back"
  printf '\n  Pick: '
  read -r p || return
  case "$p" in
    1) pkg=nano; nice="text editor" ;;
    2) pkg=lynx; nice="web" ;;
    3) pkg=htop; nice="monitor" ;;
    *) return ;;
  esac
  say ""
  say "  Installing $nice..."
  if grep -q ' /apps ' /proc/mounts 2>/dev/null; then
    if [ ! -f /apps/etc/apk/world ]; then
      mkdir -p /apps/etc/apk
      cp -a /etc/apk/keys /apps/etc/apk/ 2>/dev/null || true
      cp /etc/apk/repositories /apps/etc/apk/
      apk --root /apps --initdb add "$pkg"
    else
      apk --root /apps add "$pkg"
    fi
    st=$?
  else
    apk update && apk add "$pkg"
    st=$?
  fi
  if [ "$st" -eq 0 ]; then
    say "  Done. Stays after reboot."
  else
    say "  Could not install."
  fi
  wait_key
}

write_note() {
  np=$(note_path)
  if command -v nano >/dev/null 2>&1; then
    nano "$np"
    return
  fi
  banner
  say "  Type a note. Empty line = done."
  say "  Saved to $np"
  say ""
  : >"$np"
  while IFS= read -r line; do
    [ -z "$line" ] && break
    printf '%s\n' "$line" >>"$np"
  done
  say "  Saved."
  wait_key
}

show_files() {
  banner
  say "  Files on the stick (/stick)"
  say ""
  if [ -d /stick ]; then
    ls -la /stick 2>/dev/null || say "  (empty)"
  else
    say "  Stick not mounted."
  fi
  wait_key
}

show_status() {
  banner
  say "  Status"
  say "  name: $(cat /etc/hostname 2>/dev/null)"
  say "  ip: $(ip -4 addr 2>/dev/null | awk '/inet /{print $2}' | head -3 | tr '\n' ' ')"
  say "  net: $(ls /sys/class/net 2>/dev/null | tr '\n' ' ')"
  say "  stick: $(grep -q ' /stick ' /proc/mounts && echo yes || echo no)"
  say "  apps disk: $(grep -q ' /apps ' /proc/mounts && echo yes || echo no)"
  say "  nano: $(command -v nano >/dev/null && echo yes || echo no)"
  wait_key
}

open_web() {
  if command -v lynx >/dev/null 2>&1; then
    lynx https://example.com
  else
    banner
    say "  Install web first: Get apps -> 2"
    wait_key
  fi
}

calc() {
  banner
  say "  Type like  2+2  or  10*3  then Enter. q = back"
  while true; do
    printf '  = '
    read -r e || return
    [ "$e" = q ] && return
    awk "BEGIN { print $e }" 2>/dev/null || say "  nope"
  done
}

stop_os() {
  banner
  say "  Bye."
  sync 2>/dev/null || true
  umount /apps 2>/dev/null || true
  losetup -d /dev/loop0 2>/dev/null || true
  umount /stick 2>/dev/null || true
  if [ "$1" = reboot ]; then
    reboot -f 2>/dev/null || echo b >/proc/sysrq-trigger 2>/dev/null
  fi
  poweroff -f 2>/dev/null || halt -f 2>/dev/null || exit 0
}

while true; do
  banner
  hn=$(cat /etc/hostname 2>/dev/null || echo teos)
  say "  Hello. This computer is: $hn"
  say "  USB only. Not macOS."
  say ""
  say "  1) Change the name"
  say "  2) Get apps"
  say "  3) Write a note"
  say "  4) Files on the stick"
  say "  5) Web"
  say "  6) Status"
  say "  7) Terminal"
  say "  8) Calculator"
  say "  9) Reboot"
  say "  0) Shut down"
  printf '\n  Type a number, then Enter: '
  read -r c || c=7
  case "$c" in
    1) set_name ;;
    2) get_apps ;;
    3) write_note ;;
    4) show_files ;;
    5) open_web ;;
    6) show_status ;;
    7)
      banner
      say "  Type  exit  to come back."
      say ""
      /bin/sh
      ;;
    8) calc ;;
    9) stop_os reboot ;;
    0) stop_os halt ;;
  esac
done

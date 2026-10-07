#!/bin/sh
# TeOS home screen. Numbers, not config files.
. /etc/profile 2>/dev/null || true
export PATH="/sbin:/usr/sbin:/bin:/usr/bin"

say() { printf '%s\n' "$*"; }

banner() {
  printf '\033[2J\033[H'
  printf '\033[1;37;44m'
  printf '  TEOS                                                    \n'
  printf '\033[0m\n'
}

wait_key() {
  printf '\n  Press Enter... '
  read -r _ || true
}

note_path() {
  if [ -d /stick ] && [ -w /stick ]; then
    printf '%s' /stick/notes.txt
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
  printf '%s\n' "$n" >/stick/hostname 2>/dev/null || true
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
  if apk update && apk add "$pkg"; then
    say "  Done."
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
  say "  stick: $([ -d /stick ] && ls /stick >/dev/null 2>&1 && echo yes || echo no)"
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
  say "  8) Shut down"
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
    8)
      banner
      say "  Bye."
      poweroff -f 2>/dev/null || halt -f 2>/dev/null || exit 0
      ;;
  esac
done

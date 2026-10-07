#!/bin/sh
# TeOS first screen on Linux. No config files. Type a number.
. /etc/profile 2>/dev/null || true
export PATH="/sbin:/usr/sbin:/bin:/usr/bin"

say() { printf '%s\n' "$*"; }

banner() {
  printf '\033[2J\033[H'
  printf '\033[1;37;44m'
  printf '  TEOS                                              \n'
  printf '\033[0m\n'
}

wait_key() {
  printf '\n  Press Enter... '
  read -r _ || true
}

set_name() {
  banner
  say "  What should this computer be called?"
  say "  (letters and numbers, like  teo-laptop )"
  printf '\n  Name: '
  read -r n || return
  n=$(printf '%s' "$n" | tr -cd 'A-Za-z0-9-' | cut -c1-32)
  [ -z "$n" ] && return
  printf '%s\n' "$n" >/etc/hostname
  hostname "$n" 2>/dev/null || true
  say ""
  say "  OK. It is called $n"
  wait_key
}

get_apps() {
  banner
  say "  Apps (needs internet)"
  say ""
  say "  1) Text editor"
  say "  2) Simple web browser"
  say "  3) Back"
  printf '\n  Pick: '
  read -r p || return
  case "$p" in
    1) pkg=nano; nice="text editor" ;;
    2) pkg=lynx; nice="web browser" ;;
    *) return ;;
  esac
  say ""
  say "  Installing $nice..."
  if apk update >/tmp/teos-apk.log 2>&1 && apk add "$pkg" >>/tmp/teos-apk.log 2>&1; then
    say "  Done. Type  $pkg  later if you open Terminal."
  else
    say "  Could not. Internet missing or Alpine modules not loaded yet."
  fi
  wait_key
}

while true; do
  banner
  hn=$(cat /etc/hostname 2>/dev/null || echo teos)
  say "  Hello. This computer is: $hn"
  say "  USB only. Not macOS."
  say ""
  say "  1) Change the name"
  say "  2) Get apps"
  say "  3) Terminal"
  say "  4) Shut down"
  printf '\n  Type 1 2 3 or 4, then Enter: '
  read -r c || c=3
  case "$c" in
    1) set_name ;;
    2) get_apps ;;
    3)
      banner
      say "  Type  exit  to come back here."
      say ""
      exec /bin/sh
      ;;
    4)
      banner
      say "  Bye."
      poweroff -f 2>/dev/null || halt -f 2>/dev/null || exit 0
      ;;
  esac
done

#!/bin/sh
# shellcheck shell=sh
# Remove a user-level mediaar install created by install.sh.
#
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/ggallovalle/mediaar/main/uninstall.sh | sh
#   curl -fsSL .../uninstall.sh | sh -s -- --dry-run
#   sh uninstall.sh --dry-run
set -eu

#region logging setup
if [ "${MEDIAAR_QUIET-}" = "1" ] || [ "${MEDIAAR_QUIET-}" = "true" ]; then
  info() {
    :
  }
else
  info() {
    echo "$@" >&2
  }
fi

error() {
  echo "$@" >&2
  exit 1
}
#endregion

#region args
DRY_RUN=0
SHOW_HELP=0

usage() {
  cat <<'EOF'
Uninstall a user-level mediaar install (binary, man, zsh, desktop, icons).

Usage:
  uninstall.sh [OPTIONS]

Options:
  --dry-run    Print what would be removed without deleting files
  -h, --help   Show this help

Environment:
  MEDIAAR_INSTALL_PATH    Binary path (default: ~/.local/bin/mediaar)
  MEDIAAR_DATA_HOME       Share root (default: ${XDG_DATA_HOME:-~/.local/share})
  MEDIAAR_QUIET=1         Suppress non-error output
EOF
}

while [ "$#" -gt 0 ]; do
  case "$1" in
  --dry-run)
    DRY_RUN=1
    shift
    ;;
  -h | --help)
    SHOW_HELP=1
    shift
    ;;
  *)
    error "unknown option: $1 (try --help)"
    ;;
  esac
done

if [ "$SHOW_HELP" = "1" ]; then
  usage
  exit 0
fi

if [ "${MEDIAAR_DRY_RUN-}" = "1" ] || [ "${MEDIAAR_DRY_RUN-}" = "true" ]; then
  DRY_RUN=1
fi
#endregion

remove_path() {
  path="$1"
  if [ ! -e "$path" ] && [ ! -L "$path" ]; then
    return 0
  fi
  if [ "$DRY_RUN" = "1" ]; then
    info "+ rm -f $path"
  else
    rm -f "$path"
    info "mediaar: removed $path"
  fi
}

uninstall_mediaar() {
  data_home="${MEDIAAR_DATA_HOME:-${XDG_DATA_HOME:-$HOME/.local/share}}"
  install_path="${MEDIAAR_INSTALL_PATH:-$HOME/.local/bin/mediaar}"
  man_path="${data_home}/man/man1/mediaar.1"
  zsh_path="${data_home}/zsh/site-functions/_mediaar"
  desktop_path="${data_home}/applications/mediaar.desktop"
  icon_root="${data_home}/icons/hicolor"

  info "mediaar: uninstall binary  $install_path"
  info "mediaar: uninstall man     $man_path"
  info "mediaar: uninstall zsh     $zsh_path"
  info "mediaar: uninstall desktop $desktop_path"
  info "mediaar: uninstall icons   $icon_root/*/apps/mediaar.png"

  if [ "$DRY_RUN" = "1" ]; then
    remove_path "$install_path"
    remove_path "$man_path"
    remove_path "$zsh_path"
    remove_path "$desktop_path"
    for size in 32x32 128x128 256x256; do
      remove_path "${icon_root}/${size}/apps/mediaar.png"
    done
    info "mediaar: dry-run complete (no files removed)"
    return 0
  fi

  remove_path "$install_path"
  remove_path "$man_path"
  remove_path "$zsh_path"
  remove_path "$desktop_path"
  for size in 32x32 128x128 256x256; do
    remove_path "${icon_root}/${size}/apps/mediaar.png"
  done

  if command -v update-desktop-database >/dev/null 2>&1; then
    update-desktop-database "${data_home}/applications" >/dev/null 2>&1 || true
  fi
  if command -v gtk-update-icon-cache >/dev/null 2>&1; then
    gtk-update-icon-cache -f -t "${icon_root}" >/dev/null 2>&1 || true
  fi

  info "mediaar: uninstall complete"
}

uninstall_mediaar

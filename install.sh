#!/bin/sh
# shellcheck shell=sh
# Mediaar user install script (inspired by https://mise.run).
#
# Installs into the calling user's XDG dirs (not system-wide):
#   ~/.local/bin/mediaar
#   ~/.local/share/man/man1/mediaar.1
#   ~/.local/share/zsh/site-functions/_mediaar
#   ~/.local/share/applications/mediaar.desktop
#   ~/.local/share/icons/hicolor/*/apps/mediaar.png
#
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/ggallovalle/mediaar/main/install.sh | sh
#   curl -fsSL .../install.sh | sh -s -- --dry-run
#   MEDIAAR_VERSION=0.1.0 sh install.sh
#   sh install.sh --dry-run
set -eu

#region logging setup
if [ "${MEDIAAR_DEBUG-}" = "true" ] || [ "${MEDIAAR_DEBUG-}" = "1" ]; then
  debug() {
    echo "$@" >&2
  }
else
  debug() {
    :
  }
fi

if [ "${MEDIAAR_QUIET-}" = "1" ] || [ "${MEDIAAR_QUIET-}" = "true" ]; then
  info() {
    :
  }
else
  info() {
    echo "$@" >&2
  }
fi

warn() {
  printf '%s\n' "$*" >&2
}

error() {
  echo "$@" >&2
  exit 1
}

unsupported_arch() {
  arch="$1"
  warn "unsupported architecture: $arch"
  warn ""
  warn "mediaar does not provide prebuilt binaries for this platform."
  warn "If Rust/Cargo is available, install from source with:"
  warn "  cargo install --locked mediaar"
  warn "or:"
  warn "  cargo binstall mediaar"
  exit 1
}
#endregion

#region args
DRY_RUN=0
SHOW_HELP=0

usage() {
  cat <<'EOF'
Install mediaar for the current user (binary, zsh completion, man page, desktop entry).

Usage:
  install.sh [OPTIONS]

Options:
  --dry-run    Print what would be installed without writing files
  -h, --help   Show this help

Environment:
  MEDIAAR_VERSION         Version to install (default: latest GitHub release)
  MEDIAAR_GITHUB_REPO     owner/repo for releases (default: ggallovalle/mediaar)
  MEDIAAR_INSTALL_PATH    Binary path (default: ~/.local/bin/mediaar)
  MEDIAAR_DATA_HOME       Share root (default: ${XDG_DATA_HOME:-~/.local/share})
  MEDIAAR_TARBALL_URL     Override download URL
  MEDIAAR_DEBUG=1         Verbose logging
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

#region environment setup
get_os() {
  os="$(uname -s)"
  if [ "$os" = Darwin ]; then
    echo "macos"
  elif [ "$os" = Linux ]; then
    echo "linux"
  else
    error "unsupported OS: $os"
  fi
}

# Rust target triple used by cargo-binstall / GitHub release archives.
get_target() {
  os="$(uname -s)"
  arch="$(uname -m)"
  musl=""

  if [ "$os" = Linux ] && type ldd >/dev/null 2>/dev/null; then
    if [ "${MEDIAAR_INSTALL_MUSL-}" = "1" ] || [ "${MEDIAAR_INSTALL_MUSL-}" = "true" ]; then
      musl="1"
    elif [ "$(uname -o 2>/dev/null || true)" = "Android" ]; then
      musl="1"
    else
      libc=$(ldd /bin/ls 2>/dev/null | grep musl | head -1 | cut -d ' ' -f1 || true)
      if [ -n "$libc" ]; then
        musl="1"
      fi
    fi
  fi

  case "$os-$arch" in
  Linux-x86_64)
    if [ -n "$musl" ]; then
      echo "x86_64-unknown-linux-musl"
    else
      echo "x86_64-unknown-linux-gnu"
    fi
    ;;
  Linux-aarch64 | Linux-arm64)
    if [ -n "$musl" ]; then
      echo "aarch64-unknown-linux-musl"
    else
      echo "aarch64-unknown-linux-gnu"
    fi
    ;;
  Darwin-x86_64)
    echo "x86_64-apple-darwin"
    ;;
  Darwin-arm64 | Darwin-aarch64)
    echo "aarch64-apple-darwin"
    ;;
  *)
    unsupported_arch "$os/$arch"
    ;;
  esac
}

shasum_bin() {
  if command -v sha256sum >/dev/null 2>&1; then
    echo "sha256sum"
  elif command -v shasum >/dev/null 2>&1; then
    echo "shasum -a 256"
  else
    error "mediaar install requires sha256sum or shasum but neither is installed. Aborting."
  fi
}

http_get() {
  url="$1"
  if command -v curl >/dev/null 2>&1; then
    debug ">" curl -fsSL "$url"
    curl -fsSL "$url"
  elif command -v wget >/dev/null 2>&1; then
    debug ">" wget -qO - "$url"
    wget -qO - "$url"
  else
    error "mediaar install requires curl or wget but neither is installed. Aborting."
  fi
}

download_file() {
  url="$1"
  file="$2"
  if command -v curl >/dev/null 2>&1; then
    debug ">" curl -#fLo "$file" "$url"
    curl -#fLo "$file" "$url"
  elif command -v wget >/dev/null 2>&1; then
    debug ">" wget -qO "$file" "$url"
    stderr=$(mktemp)
    wget -O "$file" "$url" >"$stderr" 2>&1 || error "wget failed: $(cat "$stderr")"
    rm -f "$stderr"
  else
    error "mediaar install requires curl or wget but neither is installed. Aborting."
  fi
}

latest_version() {
  repo="$1"
  api="https://api.github.com/repos/${repo}/releases/latest"
  tag="$(http_get "$api" | sed -n 's/.*"tag_name": *"\(.*\)".*/\1/p' | head -1)"
  if [ -z "$tag" ]; then
    error "could not resolve latest release for ${repo}"
  fi
  echo "${tag#v}"
}

installed_mediaar_version() {
  bin="$1"
  if [ -x "$bin" ]; then
    # "Mediaar 0.1.0" or similar from --version
    installed_version="$("$bin" --version 2>/dev/null | head -n1 | awk '{print $NF}')"
    echo "${installed_version#v}"
  fi
}

#endregion

install_mediaar() {
  repo="${MEDIAAR_GITHUB_REPO:-ggallovalle/mediaar}"
  target="${MEDIAAR_INSTALL_TARGET:-$(get_target)}"
  os="$(get_os)"
  data_home="${MEDIAAR_DATA_HOME:-${XDG_DATA_HOME:-$HOME/.local/share}}"
  install_path="${MEDIAAR_INSTALL_PATH:-$HOME/.local/bin/mediaar}"
  install_dir="$(dirname "$install_path")"
  man_path="${data_home}/man/man1/mediaar.1"
  zsh_path="${data_home}/zsh/site-functions/_mediaar"
  desktop_path="${data_home}/applications/mediaar.desktop"
  icon_root="${data_home}/icons/hicolor"

  if [ -n "${MEDIAAR_VERSION-}" ]; then
    version="${MEDIAAR_VERSION#v}"
  else
    info "mediaar: resolving latest release from GitHub (${repo})..."
    version="$(latest_version "$repo")"
  fi

  if [ -n "${MEDIAAR_TARBALL_URL-}" ]; then
    tarball_url="$MEDIAAR_TARBALL_URL"
  else
    tarball_url="https://github.com/${repo}/releases/download/v${version}/mediaar-${version}-${target}.tgz"
  fi

  checksum_url="https://github.com/${repo}/releases/download/v${version}/SHA256SUMS.txt"

  skip_if_exists="${MEDIAAR_INSTALL_SKIP_IF_EXISTS-}"
  if [ "$skip_if_exists" = "1" ] || [ "$skip_if_exists" = "true" ]; then
    if [ -x "$install_path" ]; then
      existing_version="$(installed_mediaar_version "$install_path")"
      if [ -n "$existing_version" ] && [ "$existing_version" = "$version" ]; then
        info "mediaar: $install_path is already at version $version, skipping install"
        return 0
      fi
    fi
  fi

  info "mediaar: install target=$target version=$version"
  info "mediaar: binary -> $install_path"
  info "mediaar: man    -> $man_path"
  info "mediaar: zsh    -> $zsh_path"
  if [ "$os" = "linux" ]; then
    info "mediaar: desktop-> $desktop_path"
    info "mediaar: icons  -> $icon_root/*/apps/mediaar.png"
  fi
  info "mediaar: url    -> $tarball_url"

  if [ "$DRY_RUN" = "1" ]; then
    info "mediaar: dry-run complete (no files written)"
    return 0
  fi

  if [ -d "$install_path" ]; then
    error "MEDIAAR_INSTALL_PATH '$install_path' is a directory. Set it to a file path, e.g. '$HOME/.local/bin/mediaar'."
  fi

  download_dir="$(mktemp -d)"
  extract_dir="$(mktemp -d)"
  # shellcheck disable=SC2064
  trap 'rm -rf "$download_dir" "$extract_dir"' EXIT INT HUP

  info "mediaar: downloading..."
  cache_file="$download_dir/$(basename "$tarball_url")"
  download_file "$tarball_url" "$cache_file"
  debug "mediaar: tarball=$cache_file"

  if checksums="$(http_get "$checksum_url" 2>/dev/null || true)" && [ -n "$checksums" ]; then
    debug "mediaar: validating checksum"
    (
      cd "$(dirname "$cache_file")"
      echo "$checksums" | grep " $(basename "$cache_file")\$" | $(shasum_bin) -c - >/dev/null
    ) || error "checksum verification failed for $(basename "$cache_file")"
  else
    warn "mediaar: no SHA256SUMS.txt found for v${version}; skipping checksum verify"
  fi

  tar --no-same-owner -xzf "$cache_file" -C "$extract_dir"
  debug "mediaar: extracted to $extract_dir"

  # Accept either a flat archive or a single top-level directory.
  root="$extract_dir"
  if [ ! -f "$root/mediaar" ] && [ ! -f "$root/mediaar.exe" ]; then
    for candidate in "$extract_dir"/*; do
      if [ -f "$candidate/mediaar" ] || [ -f "$candidate/mediaar.exe" ]; then
        root="$candidate"
        break
      fi
    done
  fi

  bin_src=""
  if [ -f "$root/mediaar" ]; then
    bin_src="$root/mediaar"
  elif [ -f "$root/mediaar.exe" ]; then
    bin_src="$root/mediaar.exe"
  else
    error "archive did not contain a mediaar binary"
  fi

  mkdir -p "$install_dir"
  rm -f "$install_path"
  install -m 0755 "$bin_src" "$install_path"
  info "mediaar: installed binary to $install_path"

  if [ -f "$root/mediaar.1" ]; then
    mkdir -p "$(dirname "$man_path")"
    install -m 0644 "$root/mediaar.1" "$man_path"
    info "mediaar: installed man page to $man_path"
  elif [ -f "$root/share/man/man1/mediaar.1" ]; then
    mkdir -p "$(dirname "$man_path")"
    install -m 0644 "$root/share/man/man1/mediaar.1" "$man_path"
    info "mediaar: installed man page to $man_path"
  else
    warn "mediaar: man page missing from archive"
  fi

  if [ -f "$root/_mediaar" ]; then
    mkdir -p "$(dirname "$zsh_path")"
    install -m 0644 "$root/_mediaar" "$zsh_path"
    info "mediaar: installed zsh completion to $zsh_path"
  elif [ -f "$root/share/zsh/site-functions/_mediaar" ]; then
    mkdir -p "$(dirname "$zsh_path")"
    install -m 0644 "$root/share/zsh/site-functions/_mediaar" "$zsh_path"
    info "mediaar: installed zsh completion to $zsh_path"
  else
    warn "mediaar: zsh completion missing from archive"
  fi

  if [ "$os" = "linux" ]; then
    icon_file=""
    for size in 128x128 256x256 32x32; do
      icon_src=""
      if [ -f "$root/icons/${size}.png" ]; then
        icon_src="$root/icons/${size}.png"
      elif [ -f "$root/share/icons/hicolor/${size}/apps/mediaar.png" ]; then
        icon_src="$root/share/icons/hicolor/${size}/apps/mediaar.png"
      fi
      if [ -n "$icon_src" ]; then
        icon_dest="${icon_root}/${size}/apps/mediaar.png"
        mkdir -p "$(dirname "$icon_dest")"
        install -m 0644 "$icon_src" "$icon_dest"
        info "mediaar: installed icon $icon_dest"
        # Prefer a mid/large absolute path for launchers that skip theme lookup.
        if [ -z "$icon_file" ] || [ "$size" = "128x128" ]; then
          icon_file="$icon_dest"
        fi
      fi
    done

    # Unthemed fallback some pickers check before hicolor.
    if [ -n "$icon_file" ]; then
      mkdir -p "${data_home}/icons"
      install -m 0644 "$icon_file" "${data_home}/icons/mediaar.png"
      info "mediaar: installed icon ${data_home}/icons/mediaar.png"
    fi

    desktop_src=""
    if [ -f "$root/mediaar.desktop" ]; then
      desktop_src="$root/mediaar.desktop"
    elif [ -f "$root/share/applications/mediaar.desktop" ]; then
      desktop_src="$root/share/applications/mediaar.desktop"
    fi

    if [ -n "$desktop_src" ]; then
      mkdir -p "$(dirname "$desktop_path")"
      # Absolute Exec= and Icon= so GUI launchers work without PATH / icon-theme lookup.
      icon_value="${icon_file:-mediaar}"
      sed \
        -e "s|@MEDIAAR_BIN@|$install_path|g" \
        -e "s|@MEDIAAR_ICON@|$icon_value|g" \
        -e "s|^Exec=mediaar |Exec=$install_path |" \
        -e "s|^Icon=mediaar\$|Icon=$icon_value|" \
        "$desktop_src" >"$desktop_path"
      chmod 0644 "$desktop_path"
      info "mediaar: installed desktop entry to $desktop_path"
    else
      warn "mediaar: desktop entry missing from archive"
    fi

    if command -v update-desktop-database >/dev/null 2>&1; then
      update-desktop-database "${data_home}/applications" >/dev/null 2>&1 || true
    fi
    if command -v gtk-update-icon-cache >/dev/null 2>&1; then
      gtk-update-icon-cache -f -t "${icon_root}" >/dev/null 2>&1 || true
    fi
  fi

  info "mediaar: installed successfully"
}

after_finish_help() {
  install_path="${MEDIAAR_INSTALL_PATH:-$HOME/.local/bin/mediaar}"
  data_home="${MEDIAAR_DATA_HOME:-${XDG_DATA_HOME:-$HOME/.local/share}}"

  case ":${PATH}:" in
  *":$HOME/.local/bin:"* | *":$(dirname "$install_path"):"*) ;;
  *)
    info "mediaar: add $(dirname "$install_path") to your PATH, e.g.:"
    info "  echo 'export PATH=\"\$HOME/.local/bin:\$PATH\"' >> \"${ZDOTDIR:-$HOME}/.zshrc\""
    ;;
  esac

  case "${SHELL:-}" in
  */zsh)
    info "mediaar: ensure zsh loads user site-functions (once):"
    info "  echo 'fpath=(\$HOME/.local/share/zsh/site-functions \$fpath)' >> \"${ZDOTDIR:-$HOME}/.zshrc\""
    info "  # then restart the shell or run: compinit"
    ;;
  esac

  info "mediaar: try \`mediaar --help\`, \`mediaar tui\`, or open Mediaar from your app launcher"
  info "mediaar: man page: man mediaar (may need MANPATH=\$HOME/.local/share/man)"
}

install_mediaar
if [ "${MEDIAAR_INSTALL_HELP-}" != "0" ] && [ "$DRY_RUN" != "1" ]; then
  after_finish_help
fi

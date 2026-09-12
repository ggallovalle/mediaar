#!/bin/sh
# Build a GitHub-release / cargo-binstall archive for mediaar.
#
# Archive layout (flat), name: mediaar-{version}-{target}.tgz
#   mediaar                 # binary (binstall bin-dir = "{ bin }")
#   mediaar.1               # man page
#   _mediaar                # zsh completion (calls mediaar __complete_word__)
#   mediaar.desktop         # desktop entry template (@MEDIAAR_BIN@)
#   icons/32x32.png
#   icons/128x128.png
#   icons/256x256.png
#
# Usage:
#   scripts/pack-release.sh
#   scripts/pack-release.sh --skip-build
set -eu

ROOT="$(CDPATH= cd -- "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

SKIP_BUILD=0
while [ "$#" -gt 0 ]; do
  case "$1" in
  --skip-build)
    SKIP_BUILD=1
    shift
    ;;
  -h | --help)
    sed -n '1,20p' "$0"
    exit 0
    ;;
  *)
    echo "unknown option: $1" >&2
    exit 1
    ;;
  esac
done

VERSION="$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -1)"
TARGET="$(rustc -vV | sed -n 's/^host: //p')"
OUT_DIR="${MEDIAAR_DIST_DIR:-$ROOT/dist}"
STAGE="$OUT_DIR/stage"
ARCHIVE="$OUT_DIR/mediaar-${VERSION}-${TARGET}.tgz"

if [ "$SKIP_BUILD" != "1" ]; then
  echo "pack-release: building UI + release binary..."
  mise run build
fi

BIN="$ROOT/target/release/mediaar"
if [ ! -x "$BIN" ]; then
  echo "pack-release: missing $BIN (run without --skip-build)" >&2
  exit 1
fi

echo "pack-release: regenerating man page + zsh completion..."
mkdir -p packaging/man packaging/completions packaging/icons packaging/applications
"$BIN" __usage_spec__ | mise exec -- usage generate manpage -f - >packaging/man/mediaar.1

# Keep packaging/completions/_mediaar as the checked-in binary-backed script.
# Refresh icon copies from the crate icon set.
cp -f crates/mediaar/icons/32x32.png packaging/icons/32x32.png
cp -f crates/mediaar/icons/128x128.png packaging/icons/128x128.png
cp -f "crates/mediaar/icons/128x128@2x.png" packaging/icons/256x256.png

rm -rf "$STAGE"
mkdir -p "$STAGE/icons" "$OUT_DIR"

cp -f "$BIN" "$STAGE/mediaar"
chmod 755 "$STAGE/mediaar"
cp -f packaging/man/mediaar.1 "$STAGE/mediaar.1"
cp -f packaging/completions/_mediaar "$STAGE/_mediaar"
cp -f packaging/applications/mediaar.desktop "$STAGE/mediaar.desktop"
cp -f packaging/icons/32x32.png packaging/icons/128x128.png packaging/icons/256x256.png "$STAGE/icons/"

tar -C "$STAGE" -czf "$ARCHIVE" \
  mediaar \
  mediaar.1 \
  _mediaar \
  mediaar.desktop \
  icons

(
  cd "$OUT_DIR"
  if command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$(basename "$ARCHIVE")" >SHA256SUMS.txt
  else
    shasum -a 256 "$(basename "$ARCHIVE")" >SHA256SUMS.txt
  fi
)

echo "pack-release: wrote $ARCHIVE"
echo "pack-release: wrote $OUT_DIR/SHA256SUMS.txt"
cat "$OUT_DIR/SHA256SUMS.txt"

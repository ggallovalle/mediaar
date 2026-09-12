#!/usr/bin/env bash
# Measure desktop binary build time, size, and cold startup.
# Usage: benches/measure-desktop.sh <label> [output.json]
#
# Desktop startup uses MEDIAAR_BENCH_READY_FILE: the app writes this path when
# the native window shell is up (see desktop.rs).
set -euo pipefail

LABEL="${1:?label required (e.g. tauri or gpui)}"
OUT="${2:-benches/results-${LABEL}.json}"
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

BIN="$ROOT/target/release/mediaar"
UI_DIST="crates/mediaar/ui/dist"

echo "==> [$LABEL] package rebuild (release, cargo clean -p mediaar)"

UI_SIZE=0
UI_PRESENT=0
if [[ -d "$UI_DIST" ]]; then
  UI_PRESENT=1
  UI_SIZE=$(du -sb "$UI_DIST" | awk '{print $1}')
fi

cargo clean -p mediaar 2>/dev/null || true
rm -rf target/release/mediaar target/release/deps/mediaar-* target/release/.fingerprint/mediaar-* 2>/dev/null || true

START_NS=$(date +%s%N)
if [[ -d crates/mediaar/ui ]]; then
  MEDIAAR_SKIP_UI_BUILD=1 cargo build -p mediaar --release
else
  cargo build -p mediaar --release
fi
END_NS=$(date +%s%N)
BUILD_MS=$(( (END_NS - START_NS) / 1000000 ))

BIN_SIZE=$(stat -c%s "$BIN")
BIN_SIZE_HUMAN=$(numfmt --to=iec --suffix=B "$BIN_SIZE" 2>/dev/null || echo "${BIN_SIZE}B")

DEP_CRATES=$(cargo tree -p mediaar --prefix none --edges normal 2>/dev/null | awk '{print $1}' | sort -u | wc -l | tr -d ' ')

echo "==> [$LABEL] --help startup (10 runs)"
HELP_SAMPLES=()
for _ in $(seq 1 10); do
  S=$(date +%s%N)
  "$BIN" --help >/dev/null
  E=$(date +%s%N)
  HELP_SAMPLES+=($(( (E - S) / 1000000 )))
done
HELP_AVG=$(printf '%s\n' "${HELP_SAMPLES[@]}" | awk '{s+=$1} END {printf "%.1f", s/NR}')

echo "==> [$LABEL] desktop cold start via ready-file (5 runs)"
DESKTOP_SAMPLES=()
HAS_DISPLAY=0
if [[ -n "${DISPLAY:-}${WAYLAND_DISPLAY:-}" ]]; then
  HAS_DISPLAY=1
fi

measure_desktop() {
  local start end elapsed pid ready
  ready="$(mktemp /tmp/mediaar-ready-XXXXXX)"
  rm -f "$ready"
  start=$(date +%s%N)
  MEDIAAR_BENCH_READY_FILE="$ready" "$BIN" desktop \
    >/tmp/mediaar-desktop-"${LABEL}".log 2>&1 &
  pid=$!

  local found=0
  for _ in $(seq 1 200); do
    if [[ -f "$ready" ]]; then
      found=1
      break
    fi
    if ! kill -0 "$pid" 2>/dev/null; then
      break
    fi
    sleep 0.05
  done

  end=$(date +%s%N)
  elapsed=$(( (end - start) / 1000000 ))
  kill "$pid" 2>/dev/null || true
  wait "$pid" 2>/dev/null || true
  rm -f "$ready"
  sleep 0.15

  if [[ "$found" -eq 1 ]]; then
    echo "$elapsed"
  else
    echo "-1"
  fi
}

DESKTOP_AVG="null"
if [[ "$HAS_DISPLAY" -eq 1 ]]; then
  for i in $(seq 1 5); do
    sample=$(measure_desktop)
    DESKTOP_SAMPLES+=("$sample")
    echo "  run $i: ${sample}ms"
  done
  DESKTOP_AVG=$(printf '%s\n' "${DESKTOP_SAMPLES[@]}" | awk '$1>=0 {s+=$1;n++} END {if(n) printf "%.1f", s/n; else print "null"}')
fi

export LABEL GIT_SHA="$(git rev-parse --short HEAD)"
export TIMESTAMP="$(date -Iseconds)"
export BUILD_MS BIN_SIZE BIN_SIZE_HUMAN DEP_CRATES UI_PRESENT UI_SIZE
export HELP_AVG DESKTOP_AVG
export HELP_CSV="$(IFS=,; echo "${HELP_SAMPLES[*]}")"
export DESKTOP_CSV="$(IFS=,; echo "${DESKTOP_SAMPLES[*]:-}")"

python3 - "$OUT" <<'PY'
import json, sys

out_path = sys.argv[1]
# values injected via env
import os
def fnum(name, cast=float):
    v = os.environ[name]
    if v in ("null", ""):
        return None
    return cast(v)

def ilist(name):
    raw = os.environ.get(name, "")
    if not raw.strip():
        return []
    return [int(x) for x in raw.split(",") if x != ""]

out = {
    "label": os.environ["LABEL"],
    "git_sha": os.environ["GIT_SHA"],
    "timestamp": os.environ["TIMESTAMP"],
    "build_time_ms": int(os.environ["BUILD_MS"]),
    "binary_size_bytes": int(os.environ["BIN_SIZE"]),
    "binary_size_human": os.environ["BIN_SIZE_HUMAN"],
    "unique_dep_crates": int(os.environ["DEP_CRATES"]),
    "ui_dist_present": os.environ["UI_PRESENT"] == "1",
    "ui_dist_bytes": int(os.environ["UI_SIZE"]),
    "help_startup_ms_samples": ilist("HELP_CSV"),
    "help_startup_ms_avg": float(os.environ["HELP_AVG"]),
    "desktop_startup_ms_samples": ilist("DESKTOP_CSV"),
    "desktop_startup_ms_avg": fnum("DESKTOP_AVG"),
    "notes": {
        "build": "cargo clean -p mediaar then cargo build -p mediaar --release (deps warm)",
        "desktop_startup": "spawn until MEDIAAR_BENCH_READY_FILE appears",
    },
}
with open(out_path, "w") as f:
    json.dump(out, f, indent=2)
    f.write("\n")
print(json.dumps(out, indent=2))
PY

echo "==> wrote $OUT"

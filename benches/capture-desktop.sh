#!/usr/bin/env bash
# Capture desktop screenshot + memory for a mediaar binary.
# Usage: benches/capture-desktop.sh <label> <binary> <shot.png> [results.json]
set -euo pipefail

LABEL="${1:?label}"
BIN="${2:?binary path}"
SHOT="${3:?screenshot path}"
RESULTS="${4:-}"

mkdir -p "$(dirname "$SHOT")"
pkill -f "[m]ediaar desktop" 2>/dev/null || true
# also kill leftover webkit helper names if any
sleep 0.3

READY="$(mktemp /tmp/mediaar-ready-XXXX)"
rm -f "$READY"
LOG="/tmp/mediaar-${LABEL}-shot.log"

MEDIAAR_BENCH_READY_FILE="$READY" "$BIN" desktop >"$LOG" 2>&1 &
PID=$!
echo "[$LABEL] launched pid=$PID"

cleanup() {
  # Kill process group / children (Tauri may spawn WebKit helpers)
  if kill -0 "$PID" 2>/dev/null; then
    pkill -P "$PID" 2>/dev/null || true
    kill "$PID" 2>/dev/null || true
  fi
  wait "$PID" 2>/dev/null || true
  rm -f "$READY"
}
trap cleanup EXIT

# Prefer ready-file; fall back to window appearance for older Tauri builds.
READY_OK=0
for i in $(seq 1 200); do
  if [[ -f "$READY" ]]; then
    READY_OK=1
    echo "[$LABEL] ready-file after ${i} polls"
    break
  fi
  if ! kill -0 "$PID" 2>/dev/null; then
    echo "[$LABEL] process died early; log:" >&2
    cat "$LOG" >&2 || true
    exit 1
  fi
  sleep 0.05
done

find_window() {
  niri msg -j windows | python3 -c "
import sys, json
pid = int('$PID')
wins = json.load(sys.stdin)
# Match by pid, or title Mediaar, or app_id containing mediaar
cands = []
for w in wins:
    title = (w.get('title') or '')
    app = (w.get('app_id') or '')
    if w.get('pid') == pid or title == 'Mediaar' or 'mediaar' in app.lower():
        cands.append(w)
print(cands[0]['id'] if cands else '')
"
}

WIN_ID=""
for i in $(seq 1 100); do
  WIN_ID="$(find_window)"
  if [[ -n "$WIN_ID" ]]; then
    echo "[$LABEL] window id=$WIN_ID (poll $i, ready_file=$READY_OK)"
    break
  fi
  sleep 0.1
done
[[ -n "$WIN_ID" ]] || { echo "[$LABEL] window not found" >&2; exit 1; }

# Float + size for comparable screenshots (tiled columns distort layout)
niri msg action focus-window --id "$WIN_ID" >/dev/null
sleep 0.2
niri msg action move-window-to-floating >/dev/null || true
sleep 0.2
niri msg action set-window-width "960" >/dev/null || true
niri msg action set-window-height "640" >/dev/null || true
sleep 1.5

# Memory: main RSS/PSS + process-tree RSS (covers WebKit helpers)
read RSS_KB PSS_KB TREE_RSS_KB < <(python3 - <<PY
import os
from pathlib import Path

root = $PID

def children(pid):
    out = []
    for p in Path('/proc').iterdir():
        if not p.name.isdigit():
            continue
        try:
            stat = (p / 'stat').read_text().split()
            ppid = int(stat[3])
        except Exception:
            continue
        if ppid == pid:
            out.append(int(p.name))
    return out

def walk(pid):
    yield pid
    for c in children(pid):
        yield from walk(c)

def rss(pid):
    try:
        for line in Path(f'/proc/{pid}/status').read_text().splitlines():
            if line.startswith('VmRSS:'):
                return int(line.split()[1])
    except Exception:
        return 0
    return 0

def pss(pid):
    try:
        for line in Path(f'/proc/{pid}/smaps_rollup').read_text().splitlines():
            if line.startswith('Pss:'):
                return int(line.split()[1])
    except Exception:
        return 0
    return 0

pids = list(walk(root))
print(rss(root), pss(root), sum(rss(p) for p in pids))
PY
)
echo "[$LABEL] rss_kb=$RSS_KB pss_kb=$PSS_KB tree_rss_kb=$TREE_RSS_KB"

niri msg action focus-window --id "$WIN_ID" >/dev/null
sleep 0.3
rm -f "$SHOT"
niri msg action screenshot-window \
  --id "$WIN_ID" \
  --path "$SHOT" \
  --write-to-disk true \
  --show-pointer false

for i in $(seq 1 40); do
  if [[ -f "$SHOT" && "$(stat -c%s "$SHOT" 2>/dev/null || echo 0)" -gt 1000 ]]; then
    break
  fi
  sleep 0.1
done
[[ -f "$SHOT" ]] || { echo "[$LABEL] screenshot missing at $SHOT" >&2; exit 1; }
echo "[$LABEL] wrote $SHOT ($(stat -c%s "$SHOT") bytes)"

if [[ -n "$RESULTS" && -f "$RESULTS" ]]; then
  python3 - "$RESULTS" "$RSS_KB" "$PSS_KB" "$TREE_RSS_KB" "$SHOT" <<'PY'
import json, sys
from pathlib import Path
path, rss, pss, tree, shot = sys.argv[1:6]
data = json.loads(Path(path).read_text())
data["memory_rss_kb"] = int(rss)
data["memory_pss_kb"] = int(pss) if pss else None
data["memory_tree_rss_kb"] = int(tree)
shot_path = Path(shot)
try:
    data["screenshot"] = str(shot_path.relative_to(Path(path).resolve().parent))
except ValueError:
    data["screenshot"] = shot
Path(path).write_text(json.dumps(data, indent=2) + "\n")
print(json.dumps({k: data.get(k) for k in (
    "memory_rss_kb", "memory_pss_kb", "memory_tree_rss_kb", "screenshot"
)}, indent=2))
PY
fi

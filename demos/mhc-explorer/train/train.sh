#!/usr/bin/env bash
# The offline runs of mhc-explorer, part 2: each kind of mixing (plain,
# free, stochastic, orthogonal) at each depth of presets/depth.txt,
# every seed, trained for the preset's steps by the program the page
# runs (mhc-depth.xtl), cut after its model ("# -- end of the model")
# and followed by one u:r_un line per seed. Each run's record (a line
# per seed) goes to data/KIND-L.txt, and data/runs.txt says what made
# them and how long each took on this machine. Nothing is random: a
# rerun gives the same records (the times in runs.txt differ).
# Adapted from X_eTaL-fine-tuning's demos/training-at-depth/train/train.sh
# (its jobs and record files; here the program is cut, not edited).
#   demos/mhc-explorer/train/train.sh [KIND-L ...]   (default: all)
set -euo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
root="$(cd "$here/../.." && pwd)"
xetal="$("$root/scripts/xetal-ml.sh")"
XETAL_PATH="$("$root/scripts/libpath.sh")"
export XETAL_PATH
preset="$here/presets/depth.txt"
words() { grep -i "^$1 [0-9]" "$preset" | tr -cd '0-9 \n' | xargs; }
depths="$(words depths)"
seeds="$(words seeds)"
kinds=(plain free stochastic orthogonal)
runs=("$@")
if [ ${#runs[@]} -eq 0 ]; then
  for L in $depths; do for kind in "${kinds[@]}"; do runs+=("$kind-$L"); done; done
fi
work="$(mktemp -d)"; trap 'rm -rf "$work"' EXIT
mkdir -p "$work/presets" "$here/data"
cp "$preset" "$work/presets/"
log="$work/times"
one() {
  local run="$1" kind="${1%-*}" depth="${1##*-}" k
  case "$kind" in
    plain) k=1 ;; free) k=2 ;; stochastic) k=3 ;; orthogonal) k=4 ;;
    *) echo "train: no kind $kind" >&2; return 1 ;;
  esac
  sed -n '1,/^# -- end of the model/p' "$here/mhc-depth.xtl" > "$work/$run.xtl"
  for s in $(seq 1 "$seeds"); do echo "r_avel u:r_un ($k, $depth, $s, steps)" >> "$work/$run.xtl"; done
  local start=$SECONDS
  (cd "$work" && "$xetal" run --seed 1 "$run.xtl") > "$here/data/$run.txt"
  echo "$run $((SECONDS - start))" >> "$log"
  echo "train: $run, $seeds seeds, $((SECONDS - start)) s"
}
for run in "${runs[@]}"; do one "$run" & while [ "$(jobs -rp | wc -l)" -ge 4 ]; do sleep 1; done; done
wait
{
  echo "# The offline runs of mhc-explorer, part 2: train/train.sh, from mhc-depth.xtl."
  echo "steps = $(words steps)"
  echo "seeds = $seeds"
  echo "xetal_ml = $(cat "$root/XETAL_ML_COMMIT")"
  echo "xetal_libraries = $(cat "$root/XETAL_LIBRARIES_COMMIT")"
  echo "model = $(sed -n '1,/^# -- end of the model/p' "$here/mhc-depth.xtl" | git -C "$root" hash-object --stdin)  # mhc-depth.xtl up to the end of the model"
  echo "mixing = $(git -C "$root" hash-object "$root/libs/Mixing/src/Mixing.xtl")"
  echo "preset = $(git -C "$root" hash-object "$preset")"
  echo "# seconds per run on the machine that made them (not a result: four ran at once)"
  sort "$log" | sed 's/^/# /'
} > "$here/data/runs.txt"

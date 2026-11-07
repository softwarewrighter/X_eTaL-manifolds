#!/usr/bin/env bash
# Check the pinned toolchain (docs/plan.md, G1, G2): the X_eTaL-ML
# clone is at XETAL_ML_COMMIT and the X_eTaL-libraries clone at
# XETAL_LIBRARIES_COMMIT, both unchanged; the X_eTaL it pins builds,
# answers and reports that commit; a standard library imports; and the
# libraries on XETAL_PATH load from all three places: X_eTaL-ML's
# (Embed), X_eTaL-libraries' (Check, Matrix and Trig, which Manifold
# builds on). Adapted from X_eTaL-fine-tuning's scripts/check-xetal.sh:
# the libraries probed are this repository's (LoRA was the sibling's);
# docs/xetal-ML-asks.md, GM2.
#   scripts/check-xetal.sh
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
xetal="$("$root/scripts/xetal-ml.sh")"
ml="$(tr -d '[:space:]' < "$root/XETAL_ML_COMMIT")"
clone="$root/work/xetal-ml"
[ "$(git -C "$clone" rev-parse HEAD)" = "$ml" ] || { echo "check-xetal: work/xetal-ml is not at $ml" >&2; exit 1; }
short="$(cut -c1-7 "$clone/XETAL_COMMIT")"
fail() { echo "check-xetal: $1 gave '$2', expected '$3'" >&2; exit 1; }
got="$("$xetal" eval -e "'+ r_/_2 2 3 r_eshape r_ange 6")"
[ "$got" = "6 15" ] || fail eval "$got" "6 15"
"$xetal" --version | grep -q "$short" || { echo "check-xetal: xetal --version does not report $short" >&2; "$xetal" --version >&2; exit 1; }
got="$("$xetal" eval -e '"s:" u_se< "Stats"
s:m_ean 1 2 3 4')"
[ "$got" = "2.5" ] || fail Stats "$got" 2.5
got="$("$root/scripts/xt" eval -e '"em:" u_se< "Embed"
em:u_nit 2 2 r_eshape 3 4 0 2')"
[ "$got" = "$(printf '0.6 0.8\n0.0 1.0')" ] || fail "Embed (X_eTaL-ML)" "$got" "0.6 0.8 / 0.0 1.0"
got="$("$root/scripts/xt" eval -e '"ch:" u_se< "Check"
6 ch:i_s 2 * 3')"
[ "$got" = "ok" ] || fail "Check (X_eTaL-libraries)" "$got" ok
got="$("$root/scripts/xt" eval -e '"mx:" u_se< "Matrix"
mx:t_race mx:i_dentity 3')"
[ "$got" = "3" ] || fail "Matrix (X_eTaL-libraries)" "$got" 3
got="$("$root/scripts/xt" eval -e '"tr:" u_se< "Trig"
tr:a_cos 1.0')"
[ "$got" = "0.0" ] || fail "Trig (X_eTaL-libraries)" "$got" 0.0
# X_eTaL-libraries at this repository's own pin, unchanged.
libs="$(tr -d '[:space:]' < "$root/XETAL_LIBRARIES_COMMIT")"
[ "$(git -C "$root/work/xetal-libraries" rev-parse HEAD)" = "$libs" ] || { echo "check-xetal: work/xetal-libraries is not at $libs" >&2; exit 1; }
[ -z "$(git -C "$root/work/xetal-libraries" status --porcelain)" ] || { echo "check-xetal: work/xetal-libraries has local changes" >&2; exit 1; }
echo "check-xetal: ok (X_eTaL-ML ${ml:0:7}, X_eTaL $short, X_eTaL-libraries ${libs:0:7}; X_eTaL-ML's own libraries pin $(cut -c1-7 "$clone/XETAL_LIBRARIES_COMMIT"))"

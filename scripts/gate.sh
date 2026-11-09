#!/usr/bin/env bash
# The pre-commit gate, in two sizes.
#   scripts/gate.sh          affected: the cheap checks (the pinned X_eTaL,
#                            spelling, rebind, literal c_at chains,
#                            the libraries' examples, markdown), then what
#                            the change touches (scripts/affected.py plans
#                            it from the files changed since HEAD): the
#                            demos and libraries changed or importing a
#                            changed library, their pages built and checked
#                            in the browser. Before every commit.
#   scripts/gate.sh --full   everything: the tooling self-tests, this
#                            repository's shared page code, every demo,
#                            every library, every page. After a pin, before
#                            publishing the whole site, and whenever the
#                            plan says all.
# Every step of 5 seconds or more prints its time; the end lists the
# slowest three. GATE_BASE names another base than HEAD for "changed";
# GATE_PLAN gives the plan itself (lines as affected.py prints them),
# to try the gate's paths.
# Adapted from X_eTaL-fine-tuning's scripts/gate.sh: the shared step
# tests shared/ only when it holds a Cargo workspace (the teaching shell
# is not here: docs/xetal-ML-asks.md GM1); the pages are built only once
# a demo has a web/; the markdown list is every tracked or new .md but
# CLAUDE.md (its agentrail block is not ASCII) and the saga's (GM2).
#   scripts/gate.sh [--full]
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"
mode=affected
case "${1:-}" in
  --full) mode=full ;;
  "") ;;
  *) echo "usage: scripts/gate.sh [--full]" >&2; exit 2 ;;
esac
since=$SECONDS
times=""
last="start"
# A step's heading; the step before it is timed.
step() {
  if [ $((SECONDS - since)) -ge 5 ]; then printf '    (%ss)\n' $((SECONDS - since)); fi
  times="$times$((SECONDS - since)) $last\n"
  since=$SECONDS
  last="$*"
  printf '==> %s\n' "$*"
}

plan="all"
if [ "$mode" = affected ]; then
  "$root/scripts/py" "$root/scripts/affected.py" --self-test
  plan="${GATE_PLAN:-$("$root/scripts/py" "$root/scripts/affected.py")}"
  if grep -qx all <<< "$plan"; then mode=full; fi
fi
# The demos and libraries the plan names (every one in the full gate).
demos=(); libs=(); pages=0
if [ "$mode" = full ]; then
  pages=1
  while IFS= read -r s; do [ -n "$s" ] && demos+=("$s"); done < <("$root/scripts/py" "$root/scripts/demos.py" list)
  while IFS= read -r s; do [ -n "$s" ] && libs+=("$s"); done < <("$root/scripts/py" "$root/scripts/libs.py" list)
else
  while read -r kind name; do
    case "$kind" in demo) demos+=("$name") ;; lib) libs+=("$name") ;; pages) pages=1 ;; esac
  done <<< "$plan"
fi
# No page is built until a demo has a web app (none has before step
# manifold-tour: docs/plan.md, G2).
if [ $pages = 1 ] && ! ls "$root"/demos/*/web/Cargo.toml >/dev/null 2>&1; then pages=0; fi
if [ "$mode" = full ]; then echo "gate: full"; else
  printf 'gate: affected: %s demo(s), %s library(ies)%s\n' "${#demos[@]}" "${#libs[@]}" "$([ $pages = 1 ] && echo ', the pages' || true)"
fi

step "the pinned X_eTaL"
"$root/scripts/check-xetal.sh"
if [ "$mode" = full ]; then
  step "the demo tooling's self-test"
  "$root/scripts/py" "$root/scripts/demos.py" --self-test
  "$root/scripts/selftest-demos.sh"
  # The library tooling is tested only while there is a library to use it.
  if [ ${#libs[@]} -gt 0 ]; then
    step "the library tooling's self-test"
    "$root/scripts/selftest-libs.sh"
  fi
  # This repository's shared page code, once there is any (a Cargo
  # workspace in shared/); the teaching shell itself is X_eTaL-ML's (GM1).
  if [ -f "$root/shared/Cargo.toml" ]; then
    step "shared/"
    (cd "$root/shared" && cargo test -q >/dev/null 2>&1 && cargo check -q --target wasm32-unknown-unknown) \
      || { (cd "$root/shared" && cargo test -q); echo "FAIL: shared/"; exit 1; }
    echo "ok: shared/"
  fi
fi
if [ $pages = 1 ]; then
  # The site built fresh (not tracked; just publish publishes it), so the
  # browser checks below run against this checkout's pages: every page
  # in the full gate, the affected demos' otherwise.
  step "the pages built"
  if [ "$mode" = full ]; then "$root/scripts/build-pages.sh" >/dev/null
  else "$root/scripts/build-pages.sh" ${demos[@]+"${demos[@]}"} >/dev/null; fi
fi
if [ ${#demos[@]} -gt 0 ]; then
  step "the demos: ${demos[*]}"
  "$root/scripts/test-demos.sh" "${demos[@]}"
fi
if [ ${#libs[@]} -gt 0 ]; then
  step "the libraries: ${libs[*]}"
  "$root/scripts/test-libs.sh" "${libs[@]}"
  step "their doc examples"
  "$root/scripts/doc-test.sh" "${libs[@]}"
fi
step "the cheap checks"
"$root/scripts/py" "$root/scripts/check-examples.py"
# The pages' glossary is made from docs/glossary.md and never drifts from it.
"$root/scripts/py" "$root/scripts/glossary.py" --check
# Each name bound once in its scope (X_eTaL lang-choices M1).
"$root/scripts/py" "$root/scripts/check-rebind.py" --self-test
"$root/scripts/py" "$root/scripts/check-rebind.py"
# Concise X_eTaL: number literals strand, so a shape is written
# 8 13 2 r_eshape x, never (8 c_at 13 c_at 2) r_eshape x (c_at is for names).
if git ls-files '*.xtl' '*.xtlm' '*.rs' '*.md' | grep -v -E '^(CHANGES.md|docs/plan.md|\.agentrail)' | xargs grep -n -E '\(([0-9]+ c_at )+[0-9]+\)' ; then
  echo "FAIL: literal c_at chains (write the numbers side by side)"; exit 1
fi
# American spellings only (the checker checks itself first).
"$root/scripts/py" "$root/scripts/check-spelling.py" --self-test
"$root/scripts/py" "$root/scripts/check-spelling.py"
# ASCII-only markdown: every .md tracked or new, but CLAUDE.md (AGENTS.md
# is its link; the agentrail briefing it holds is not ASCII) and the saga's.
md=()
while IFS= read -r f; do
  case "$f" in CLAUDE.md|AGENTS.md|.agentrail/*|.agentrail-archive/*|work/*|pages/*) continue ;; esac
  md+=("$f")
done < <({ git ls-files '*.md'; git ls-files --others --exclude-standard '*.md'; } | sort -u)
for f in "${md[@]}"; do sw-markdown-checker -f "$f" >/dev/null || { sw-markdown-checker -f "$f"; exit 1; }; done
echo "markdown: ok (${#md[@]} files)"
step "done"
printf 'gate: %s, %ss; the slowest:\n' "$mode" "$SECONDS"
printf "$times" | sort -rn | head -3 | sed 's/^/  /'
echo "gate: ok"

#!/usr/bin/env bash
# Serve the built pages/ locally as GitHub Pages will, under
# /X_eTaL-manifolds/: http://127.0.0.1:PORT/X_eTaL-manifolds/ (default 3285).
#   scripts/serve-pages.sh [PORT]
# Adapted from X_eTaL-fine-tuning's scripts/serve-pages.sh (the base path and the port, 3285, are this repository's; docs/xetal-ML-asks.md, GM2).
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
port="${1:-3285}"
site="$root/target/serve-pages"
mkdir -p "$site"
ln -sfn "$root/pages" "$site/X_eTaL-manifolds"
echo "serving http://127.0.0.1:$port/X_eTaL-manifolds/ (Ctrl-C stops)"
exec "$root/scripts/py" -m http.server "$port" --bind 127.0.0.1 --directory "$site"

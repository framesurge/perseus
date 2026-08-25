#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
MD="$ROOT/portfolio/docs/PORTFOLIO_COMPONENT_ECOSYSTEM.md"
CSS="$ROOT/portfolio/docs/PORTFOLIO_COMPONENT_ECOSYSTEM.print.css"
HTML="/tmp/PORTFOLIO_COMPONENT_ECOSYSTEM.html"
PDF="$ROOT/portfolio/docs/PORTFOLIO_COMPONENT_ECOSYSTEM.pdf"

if ! command -v pandoc >/dev/null 2>&1; then
  echo "pandoc is required" >&2
  exit 1
fi

if [ ! -x /opt/google/chrome/chrome ]; then
  echo "headless Chrome is required at /opt/google/chrome/chrome" >&2
  exit 1
fi

pandoc "$MD" \
  --from markdown \
  --to html5 \
  --standalone \
  --embed-resources \
  --toc \
  --css "$CSS" \
  -o "$HTML"

FILE_URL="file://$HTML"
/opt/google/chrome/chrome \
  --headless=new \
  --no-sandbox \
  --disable-gpu \
  --print-to-pdf-no-header \
  --print-to-pdf="$PDF" \
  "$FILE_URL"

echo "wrote $PDF"

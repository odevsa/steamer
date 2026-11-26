#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SRC_SVG="${ROOT_DIR}/assets/logo.svg"
FRONTEND_STATIC="${ROOT_DIR}/frontend/static"
UMBREL_DIR="${ROOT_DIR}/umbrel"

if [ ! -f "$SRC_SVG" ]; then
  echo "Source SVG not found: $SRC_SVG" >&2
  exit 1
fi

render_png() {
  local size="$1"
  local dest="$2"
  if command -v rsvg-convert >/dev/null 2>&1; then
    rsvg-convert -w "$size" -h "$size" "$SRC_SVG" -o "$dest"
  elif command -v magick >/dev/null 2>&1; then
    magick -background none -density 384 "$SRC_SVG" -resize "${size}x${size}" "$dest"
  elif command -v convert >/dev/null 2>&1; then
    convert -background none -density 384 "$SRC_SVG" -resize "${size}x${size}" "$dest"
  else
    echo "Missing SVG renderer (install librsvg2-bin / rsvg-convert or ImageMagick)" >&2
    exit 1
  fi
}

mkdir -p "$FRONTEND_STATIC" "$UMBREL_DIR"

cp "$SRC_SVG" "${FRONTEND_STATIC}/logo.svg"
cp "$SRC_SVG" "${FRONTEND_STATIC}/favicon.svg"
cp "$SRC_SVG" "${UMBREL_DIR}/icon.svg"

render_png 32 "${FRONTEND_STATIC}/favicon-32x32.png"
render_png 180 "${FRONTEND_STATIC}/apple-touch-icon.png"
render_png 192 "${FRONTEND_STATIC}/icon-192.png"
render_png 512 "${FRONTEND_STATIC}/icon-512.png"
render_png 256 "${UMBREL_DIR}/icon.png"

TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

render_png 16 "${TMP_DIR}/16.png"
render_png 32 "${TMP_DIR}/32.png"
render_png 48 "${TMP_DIR}/48.png"

if command -v magick >/dev/null 2>&1; then
  magick "${TMP_DIR}/16.png" "${TMP_DIR}/32.png" "${TMP_DIR}/48.png" "${FRONTEND_STATIC}/favicon.ico"
elif command -v convert >/dev/null 2>&1; then
  convert "${TMP_DIR}/16.png" "${TMP_DIR}/32.png" "${TMP_DIR}/48.png" "${FRONTEND_STATIC}/favicon.ico"
elif command -v python3 >/dev/null 2>&1; then
  python3 - "$TMP_DIR" "${FRONTEND_STATIC}/favicon.ico" <<'PY'
import sys
from pathlib import Path
from PIL import Image

tmp_dir = Path(sys.argv[1])
out_path = Path(sys.argv[2])
base = Image.open(tmp_dir / "48.png").convert("RGBA")
base.save(out_path, format="ICO", sizes=[(16, 16), (32, 32), (48, 48)])
PY
else
  echo "Missing ICO converter (install ImageMagick or Python Pillow)" >&2
  exit 1
fi

echo "Assets generated from ${SRC_SVG}:"
echo "  - ${FRONTEND_STATIC}/logo.svg"
echo "  - ${FRONTEND_STATIC}/favicon.svg"
echo "  - ${FRONTEND_STATIC}/favicon.ico"
echo "  - ${FRONTEND_STATIC}/favicon-32x32.png"
echo "  - ${FRONTEND_STATIC}/apple-touch-icon.png"
echo "  - ${FRONTEND_STATIC}/icon-192.png"
echo "  - ${FRONTEND_STATIC}/icon-512.png"
echo "  - ${UMBREL_DIR}/icon.svg"
echo "  - ${UMBREL_DIR}/icon.png"

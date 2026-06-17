#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
CORES="$ROOT/src-tauri/cores"
URL="https://buildbot.libretro.com/nightly/linux/x86_64/latest/mgba_libretro.so.zip"
TMP="$(mktemp -d)"

cleanup() { rm -rf "$TMP"; }
trap cleanup EXIT

mkdir -p "$CORES"
echo "→ Descargando mGBA Libretro…"
curl -fL --retry 3 -o "$TMP/mgba.zip" "$URL"
unzip -o "$TMP/mgba.zip" -d "$TMP"
install -m 755 "$TMP/mgba_libretro.so" "$CORES/mgba_libretro.so"
echo "✓ Instalado: $CORES/mgba_libretro.so"
file "$CORES/mgba_libretro.so"

#!/usr/bin/env bash
# Fits a still to the TypeSafe image limits (https://docs.typesafe.ai/images.md,
# "Limits" and "Preparing images"): longest side at most 1568 px, no metadata,
# baseline JPEG. Usage: prepare-still.sh <in.jpg> <out.jpg>
set -euo pipefail
magick "$1" -resize '1568x1568>' -strip -interlace none -quality 90 "$2"
magick identify -format '%wx%h %b\n' "$2"

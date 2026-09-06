#!/bin/sh
# Optional Linux Vulkan capture backend. The QA receipt records the actual GPU.
exec chromium --enable-gpu --use-gl=angle --use-angle=vulkan \
  --enable-features=Vulkan --disable-vulkan-surface "$@"

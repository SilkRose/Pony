#!/usr/bin/env sh

cover_count=$(find "../stories/" -type f -name "cover*.*" -not -name "*concept*" -not -name "*.xcf" -not -name "*upscaled.*" | wc -l)

echo "$cover_count"

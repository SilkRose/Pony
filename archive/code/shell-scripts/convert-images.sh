#!/usr/bin/env sh

set -o errexit
set -o nounset

image_files=$(find "./" -type f -name "*.png")

for file in $image_files; do
	webp=$(echo "$file" | sed 's/\.png/\.webp/')
	magick "$file" -quality 100 -define webp:lossless=true "$webp"
	rm "$file"
done

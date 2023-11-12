#!/usr/bin/env sh

set -o errexit
set -o nounset

location="$1"

if [ -d "$location" ]; then
	cover_count=0
	story_folders=$(find "$location" -mindepth 1 -maxdepth 1 -type d)
	for folder in $story_folders; do
		cover_folder_count=$(find "$folder" -type f -name "*cover*.*" \
			-not -name "*concept*" \
			-not -name "*.xcf" \
			-not -name "*upscaled*.*" | wc -l)

		if [ "$cover_folder_count" -gt 0 ]; then
			cover_count=$((cover_count + 1))
		fi
	done
	echo "$cover_count"
else
	echo 0
fi

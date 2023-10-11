#!/usr/bin/env sh

location=$1

if [ -n "$location" ]; then
	cover_count=$(find "$location" -type f -name "*cover*.*" \
		-not -name "*concept*" \
		-not -name "*.xcf" \
		-not -name "*upscaled*.*" | wc -l)

	echo "$cover_count"
else
	echo 0
fi

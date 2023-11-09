#!/usr/bin/env sh

input=$1

if [ -n "$input" ]; then
	jq --tab . "$input" > "$input.tmp" && mv "$input.tmp" "$input"
else
	find . -type f -name "*.json" | while IFS= read -r file; do
		jq --tab . "$file" > "$file.tmp" && mv "$file.tmp" "$file"
	done
fi

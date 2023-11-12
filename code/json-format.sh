#!/usr/bin/env sh

set -o errexit
set -o nounset

if ! command -v jq > /dev/null 2>&1; then
	echo "jq is not installed. Please install it."
	exit 1
fi

find . -type f -name "*.json" | while IFS= read -r file; do
	jq --tab . "$file" > "$file.tmp" && mv "$file.tmp" "$file"
done

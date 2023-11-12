#!/usr/bin/env sh

set -o errexit
set -o nounset

if ! command -v fimd > /dev/null 2>&1; then
	echo "fimd is not installed. Please install it."
	exit 1
fi

if [ -d ./publish ]; then
	rm -rf ./publish
fi

mkdir ./publish

md_files=$(find "../" -type f -name "*.md" \
	-not -name "ideas.md" \
	-not -name "README.md" \
	-not -name "names.md" \
	-not -path "*/archive/*" \
	-path "*/stories/*" -a \
	-not -path "*/code/*" -o \
	-path "*/flash-fiction/*" -a \
	-not -path "*/code/*")

for file in $md_files; do
	path=$(echo "$file" | awk '{sub("..", "./publish")}; {sub(".md", ".txt")} 1')
	mkdir -p "$(dirname "$path")"
	touch "$path"
	fimd "$file" "$path" > /dev/null
	echo "Converted $file"
done

#!/usr/bin/env sh

set -o errexit
set -o nounset

stories_location=$1
flash_fiction_location=$2

total_word_count=0

if [ -d "$stories_location" ]; then
	md_files=$(find "$stories_location" -type f -name "*.md" \
		-not -name "*-meta.md" \
		-not -name "ideas.md" \
		-not -name "names.md" \
		-not -wholename "*darling-and-date/meta.md")
	for file in $md_files; do
		word_count=$(sed '/<center>\*\*\*<\/center>\|<p align="center">\*\*\*<\/p>/d' < "$file" \
			| tr -d "#>\-*–|—" | sed '/<!--.*-->/d' | wc -w)
		total_word_count=$((total_word_count + word_count))
	done
fi

if [ -d "$flash_fiction_location" ]; then
	md_files=$(find "$flash_fiction_location" -type f -name "*.md")
	for file in $md_files; do
		word_count=$(sed '/<center>\*\*\*<\/center>\|<p align="center">\*\*\*<\/p>/d' < "$file" \
			| tr -d "#>\-*–|—" | sed '/<!--.*-->/d' | wc -w)
		link_word_count=$(grep -E '\[.*\]\(.*\)' "$file" | wc -w)
		word_count=$((word_count - link_word_count))
		total_word_count=$((total_word_count + word_count))
	done
fi

echo "$total_word_count"

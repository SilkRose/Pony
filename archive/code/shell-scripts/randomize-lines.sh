#!/usr/bin/env sh

set -o errexit
set -o nounset

file="$1"

line_one=$(head -n 1 "$file")
lines=$(sed '1d;/^$/d' "$file")
shuffled_lines=$(echo "$lines" | shuf)
shuffled_lines_with_empty_lines=$(echo "$shuffled_lines" | sed G)
result="$line_one\n\n$shuffled_lines_with_empty_lines"

echo "$result" > "$file"

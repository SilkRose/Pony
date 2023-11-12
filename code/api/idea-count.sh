#!/usr/bin/env sh

set -o errexit
set -o nounset

location=$1

if [ -f "$location/ideas.md" ]; then
	idea_count=$(grep -c "^## " < "$location/ideas.md")
	echo "$idea_count"
else
	echo 0
fi

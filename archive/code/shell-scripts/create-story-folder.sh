#!/usr/bin/env sh

set -o errexit
set -o nounset

story_name="$1"
synopsis="$2"
path_name=$(echo "$story_name" | sed -e 's/\([A-Z]\)/\L\1/g' -e 's/ /-/g')

if [ -d "../stories/$path_name" ]; then
	echo "Story folder already exists."
	exit 1
fi

mkdir "../stories/$path_name"

printf "%s\n\n" "# $story_name" > "../stories/$path_name/$path_name.md"

sed -e "s/# Title/# $story_name/g" \
	-e "/## Story:/!b;n;c\\[$story_name](.\\/$path_name.md)" \
	< "../markdown-templates/story-one-shot.md" > "../stories/$path_name/$path_name-meta.md"

if [ -n "$synopsis" ]; then
	sed -i "/## Synopsis:/!b;n;c\\$synopsis" "../stories/$path_name/$path_name-meta.md"
fi

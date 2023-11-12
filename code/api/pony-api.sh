#!/usr/bin/env sh

set -o errexit
set -o nounset

pony_folder=$(./location-finder.sh pony)
stories_folder=$(./location-finder.sh stories)
flash_fiction_folder=$(./location-finder.sh flash-fiction)

format_count() {
	echo "$1" | sed -e :a -e "s/\(.*[0-9]\)\([0-9]\{3\}\)/\1,\2/;ta"
}

covers=$(format_count "$(./cover-count.sh "$stories_folder")")
flash_fiction=$(format_count "$(./flash-fiction-count.sh "$flash_fiction_folder")")
ideas=$(format_count "$(./idea-count.sh "$stories_folder")")
names=$(format_count "$(./name-count.sh "$stories_folder")")
stories=$(format_count "$(./story-count.sh "$stories_folder")")
words=$(format_count "$(./word-count.sh "$stories_folder" "$flash_fiction_folder")")

jq --null-input --tab \
	--arg covers "$covers" \
	--arg flash_fiction "$flash_fiction" \
	--arg ideas "$ideas" \
	--arg names "$names" \
	--arg stories "$stories" \
	--arg words "$words" \
	'{
		covers: $covers,
		flash_fiction: $flash_fiction,
		ideas: $ideas,
		names: $names,
		stories: $stories,
		words: $words
	}'

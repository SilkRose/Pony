#!/usr/bin/env sh

count_and_format() {
	script="$1"
	directory="$2"
	count=$(sh "$script" "$(./location-finder.sh "$directory")")
	echo "$count" | sed -e :a -e 's/\(.*[0-9]\)\([0-9]\{3\}\)/\1,\2/;ta'
}

covers=$(count_and_format "./cover-count.sh" "stories")
flash_fiction=$(count_and_format "./flash-fiction-count.sh" "flash-fiction")
ideas=$(count_and_format "./idea-count.sh" "ideas")
names=$(count_and_format "./name-count.sh" "names")
stories=$(count_and_format "./story-count.sh" "stories")
words=$(sh "./word-count.sh" "$(./location-finder.sh "stories")" "$(./location-finder.sh "flash-fiction")" \
	| sed -e :a -e 's/\(.*[0-9]\)\([0-9]\{3\}\)/\1,\2/;ta')

jq --null-input --tab \
	--arg covers "$covers" \
	--arg flash_fiction "$flash_fiction" \
	--arg ideas "$ideas" \
	--arg names "$names" \
	--arg stories "$stories" \
	--arg words "$words" \
	'{
		"covers": $covers,
		"flash_fiction": $flash_fiction,
		"ideas": $ideas,
		"names": $names,
		"stories": $stories,
		"words": $words
	}'

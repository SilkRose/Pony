#!/usr/bin/env sh

set -o errexit
set -o nounset

if [ -d ./dist ]; then
	rm -rf ./dist
fi

if [ -d ./pony-temp ]; then
	rm -rf ./pony-temp
fi

git clone --quiet --depth 1 --branch mane https://github.com/SilkRose/Pony.git pony-temp
git clone --quiet --depth 1 --branch api https://github.com/SilkRose/Pony.git dist

mkdir -p ./dist/api/v1

touch ./dist/.nojekyll
printf pony.silkrose.dev > ./dist/CNAME

cd ./pony-temp

format_count() {
	echo "$1" | sed -e :a -e "s/\(.*[0-9]\)\([0-9]\{3\}\)/\1,\2/;ta"
}

cover_count() {
	cover_count=0
	story_folders=$(find ./stories -mindepth 1 -maxdepth 1 -type d)
	for folder in $story_folders; do
		cover_folder_count=$(find "$folder" -type f -name "*cover*.*" \
			-not -name "*concept*" \
			-not -name "*.xcf" \
			-not -name "*upscaled*.*" | wc -l)
		if [ "$cover_folder_count" -gt 0 ]; then
			cover_count=$((cover_count + 1))
		fi
	done
	if [ -e "./stories/best-sister-never/best-sister-ever-cover.png" ]; then
		cover_count=$((cover_count + 1))
	fi
	format_count "$cover_count"
}

flash_fiction_count() {
	flash_fiction_count=$(find ./flash-fiction -type f -name "*.md" | wc -l)
	format_count "$flash_fiction_count"
}

count_lines_by() {
	count=$(grep -c "$1" < "$2")
	format_count "$count"
}

story_count() {
	story_count=$(find ./stories -mindepth 1 -maxdepth 1 -type d | wc -l)
	format_count "$story_count"
}

word_count() {
	total_word_count=0
	md_files=$(find ./stories ./flash-fiction -type f -name "*.md" \
		-not -name "*-meta.md" \
		-not -name "ideas.md" \
		-not -name "names.md")
	for file in $md_files; do
		word_count=$(sed '/\[.*\](.*\(\w\|\s\)*|<!--/,/-->/d' "$file" \
			| tr -d "#>\-*–|—" | tr -d '[:punct:]' | wc -w)
		total_word_count=$((total_word_count + word_count))
	done
	format_count "$total_word_count"
}

covers=$(cover_count)
flash_fiction=$(flash_fiction_count)
ideas=$(count_lines_by "^## " "./stories/ideas.md")
names=$(count_lines_by "\- ." "./stories/names.md")
stories=$(story_count)
words=$(word_count)

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
	}' > ../dist/api/v1/pony.json

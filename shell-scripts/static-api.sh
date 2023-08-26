#!/usr/bin/env sh

if [ -d "./dist" ]; then
    rm -rf "./dist"
fi

mkdir "./dist"

covers=$(sh "./cover-count.sh" | numfmt --grouping)
flash_fictions=$(sh "./flash-fiction-count.sh" | numfmt --grouping)
ideas=$(sh "./idea-count.sh" | numfmt --grouping)
names=$(sh "./name-count.sh" | numfmt --grouping)
stories=$(sh "./story-count.sh" | numfmt --grouping)
words=$(sh "./word-count.sh" | numfmt --grouping)

json=$(cat "api-template.json")

json=$(echo "$json" | sed -e "s/\$covers/$covers/g" \
    -e "s/\$flash-fictions/$flash_fictions/g" \
    -e "s/\$ideas/$ideas/g" \
    -e "s/\$names/$names/g" \
    -e "s/\$stories/$stories/g" \
    -e "s/\$words/$words/g")

echo "$json" > "./dist/pony.json"

#!/usr/bin/env sh

if [ -d "./dist" ]; then
    rm -rf ./dist
fi

mkdir "./dist"

covers=$(sh "./cover-count.sh" | xargs printf "%'d")
flash_fictions=$(sh "./flash-fiction-count.sh" | xargs printf "%'d")
ideas=$(sh "./idea-count.sh" | xargs printf "%'d")
names=$(sh "./name-count.sh" | xargs printf "%'d")
stories=$(sh "./story-count.sh" | xargs printf "%'d")
words=$(sh "./word-count.sh" | xargs printf "%'d")

json=$(cat "api-template.json")

json=$(echo "$json" | sed -e "s/\$covers/$covers/g" \
    -e "s/\$flash-fictions/$flash_fictions/g" \
    -e "s/\$ideas/$ideas/g" \
    -e "s/\$names/$names/g" \
    -e "s/\$stories/$stories/g" \
    -e "s/\$words/$words/g")

echo "$json" > "./dist/pony.json"

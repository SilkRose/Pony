#!/usr/bin/env sh

if [ -d "./dist" ]; then
	rm -rf "./dist"
fi

mkdir "./dist"

covers=$(sh "./cover-count.sh" | sed ':a;s/\B[0-9]\{3\}\>/,&/;ta')
flash_fictions=$(sh "./flash-fiction-count.sh" | sed ':a;s/\B[0-9]\{3\}\>/,&/;ta')
ideas=$(sh "./idea-count.sh" | sed ':a;s/\B[0-9]\{3\}\>/,&/;ta')
names=$(sh "./name-count.sh" | sed ':a;s/\B[0-9]\{3\}\>/,&/;ta')
stories=$(sh "./story-count.sh" | sed ':a;s/\B[0-9]\{3\}\>/,&/;ta')
words=$(sh "./word-count.sh" | sed ':a;s/\B[0-9]\{3\}\>/,&/;ta')

json=$(cat "api-template.json")

json=$(echo "$json" | sed -e "s/\$covers/$covers/g" \
	-e "s/\$flash-fictions/$flash_fictions/g" \
	-e "s/\$ideas/$ideas/g" \
	-e "s/\$names/$names/g" \
	-e "s/\$stories/$stories/g" \
	-e "s/\$words/$words/g")

echo "$json" > "./dist/pony.json"

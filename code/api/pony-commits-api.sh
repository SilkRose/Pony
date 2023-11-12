#!/usr/bin/env sh

set -o errexit
set -o nounset

status=$1

git_clone_url=$(jq -r .git_clone_url ./variables.json)
pony_commits_json_url=$(jq -r .pony_commits_json_url ./variables.json)

if [ -d ./pony-temp ]; then
	rm -rf ./pony-temp
fi

git clone --quiet "$git_clone_url" pony-temp

cd ./pony-temp

commits=$(git log mane --format="format:%H\n%s\n%ct")
pony_commits=$(curl --silent "$pony_commits_json_url")

get_stats() {
	hash=$1
	if [ "$status" != "rebuild" ] \
		&& echo "$pony_commits" | jq --arg hash "$hash" 'map(.hash) | contains([$hash])' | grep -q "true"; then
		echo "$pony_commits" | jq --arg hash "$hash" '.[] | select(.hash == $hash) | .stats'
	else
		git checkout --force --quiet "$hash"
		if [ -d ./code ]; then
			rm -rf ./code
		fi
		mkdir ./code
		cp -r ../*.sh ./code/
		cd ./code
		mkdir -p ./dist/api/v1
		sh ./pony-api.sh > ./dist/api/v1/pony.json
		cd ..
		jq . ./code/dist/api/v1/pony.json
	fi
}

echo "$commits" \
	| while IFS= read -r hash && IFS= read -r subject && IFS= read -r timestamp; do
		jq --null-input \
			--arg hash "$hash" \
			--arg subject "$subject" \
			--arg timestamp "$timestamp" \
			--argjson stats "$(get_stats "$hash")" \
			'{
				hash: $hash,
				subject: $subject,
				unix_time: ($timestamp | tonumber),
				stats: $stats
			}'
	done | jq -n --tab '[inputs]'

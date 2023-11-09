#!/usr/bin/env sh

if [ -d "./dist" ]; then
	rm -rf "./dist"
fi

mkdir -p "./dist/api/v1"

touch "./dist/.nojekyll"
printf "pony.silkrose.dev" > "./dist/CNAME"

sh "./pony-api.sh" > "./dist/api/v1/pony.json"

sha256sum build-api.sh \
	cover-count.sh \
	flash-fiction-count.sh \
	idea-count.sh \
	location-finder.sh \
	name-count.sh \
	pony-api.sh \
	pony-commits-api.sh \
	story-count.sh \
	word-count.sh > "./dist/shell-script-hashes"

pony_commits_json_url=$(jq -r ".pony_commits_json_url" "./variables.json")
shell_script_hashes_url=$(jq -r ".shell_script_hashes_url" "./variables.json")

remote_hash=$(curl --silent "$shell_script_hashes_url" | sha256sum | awk '{print $1}')
local_hash=$(sha256sum "./dist/shell-script-hashes" | awk '{print $1}')

if ! curl --output /dev/null --silent --head --fail "$pony_commits_json_url" \
	|| ! curl --output /dev/null --silent --head --fail "$shell_script_hashes_url"; then
	status="rebuild"
elif [ "$remote_hash" != "$local_hash" ]; then
	status="rebuild"
fi

sh "./pony-commits-api.sh" "$status" > "./dist/api/v1/pony-commits.json"

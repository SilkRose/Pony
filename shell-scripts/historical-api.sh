#!/usr/bin/env sh

if [ -d "./pony-temp" ]; then
	rm -rf "./pony-temp"
fi

git clone https://github.com/SilkRose/Pony.git pony-temp

cd "./pony-temp" || exit 1

commits=$(git log mane --format='format:%H\n%s\n%ct')

get_stats() {
	hash=$1
	git checkout --force --quiet "$hash"
	if [ -d "./shell-scripts" ]; then
		rm -rf "./shell-scripts"
	fi
	mkdir "./shell-scripts"
	cp -r ../*.sh "./shell-scripts/"
	cd "./shell-scripts" || exit 1
	sh "./static-api.sh"
	cd ..
	jq . -c "./shell-scripts/dist/pony.json"
}

echo "$commits" \
	| while IFS= read -r hash && IFS= read -r subject && IFS= read -r timestamp; do
		jq --null-input -c \
			--arg hash "$hash" \
			--arg subject "$subject" \
			--arg timestamp "$timestamp" \
			--argjson stats "$(get_stats "$hash")" \
			'{
				"hash": $hash,
				"subject": $subject,
				"unix_time": $timestamp,
				"stats": $stats
			} | to_entries
			| map(.key |= gsub("_"; "-"))
			| from_entries'
	done | jq -n -c '[inputs]' > "../dist/pony-hisorical.json"

#!/usr/bin/env sh

set -o errexit
set -o nounset

location=$1

if [ -f "$location/names.md" ]; then
	name_count=$(grep -c "\- ." < "$location/names.md")
	echo "$name_count"
else
	echo 0
fi

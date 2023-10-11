#!/usr/bin/env sh

location=$1

if [ -n "$location" ]; then
	name_count=$(grep -c "\- ." < "$location")
	echo "$name_count"
else
	echo 0
fi

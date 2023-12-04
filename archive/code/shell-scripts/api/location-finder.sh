#!/usr/bin/env sh

set -o errexit
set -o nounset

location="$1"

if [ -z "$location" ]; then
	echo "No parameter passed to find, exiting now." >&2
	exit 1
fi

parent_dir=$(pwd | awk -F '/' '{print $(NF-1)}')

case "$parent_dir" in
	"Pony" | "pony-temp")
		pony=".."
		;;
	"code")
		pony="../.."
		;;
	*)
		echo "Invalid directory: $parent_dir" >&2
		exit 1
		;;
esac

case "$location" in
	"stories" | "flash-fiction")
		if [ -d "$pony/$location" ]; then
			echo "$pony/$location"
		elif [ -d "$pony/src/$location" ]; then
			echo "$pony/src/$location"
		fi
		;;
	"pony")
		echo "$pony"
		;;
	*)
		echo "Invalid option: $location" >&2
		exit 1
		;;
esac

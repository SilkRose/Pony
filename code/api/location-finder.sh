#!/usr/bin/env sh

location="$1"

if [ -z "$location" ]; then
	echo "No parameter passed to find, exiting now." >&2
	exit 1
fi

parent_dir="$(basename "$(dirname "$(pwd)")")"

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
		if [ -d "$pony/$location/" ]; then
			echo "$pony/$location/"
		elif [ -d "$pony/src/$location/" ]; then
			echo "$pony/src/$location/"
		fi
		;;
	"ideas" | "names")
		if [ -f "$pony/stories/$location.md" ]; then
			echo "$pony/stories/$location.md"
		elif [ -f "$pony/src/stories/$location.md" ]; then
			echo "$pony/src/stories/$location.md"
		fi
		;;
	*)
		echo "Invalid option: $location" >&2
		exit 1
		;;
esac

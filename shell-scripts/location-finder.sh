#!/usr/bin/env sh

location="$1"

if [ -z "$location" ]; then
    echo "No parameter passed to find, exiting now."
    exit
fi

case "$location" in
  "stories"| "flash-fiction")
    if [ -d "../$location/" ]; then
        echo "../$location/"
    elif [ -d "../src/$location/" ]; then
        echo "../src/$location/"
    else
        echo "Unable to find $location folder, exiting now."
        exit
    fi
    ;;
  "ideas" | "names")
    if [ -f "../stories/$location.md" ]; then
        echo "../stories/$location.md"
    elif [ -f "../src/stories/$location.md" ]; then
        echo "../src/stories/$location.md"
    else
        echo "none"
    fi
    ;;
  *)
    echo "Invalid option: $location"
    exit
    ;;
esac

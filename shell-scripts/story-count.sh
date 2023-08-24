#!/usr/bin/env sh

story_count=$(find "../stories/" -mindepth 1 -maxdepth 1 -type d | wc -l)

echo "$story_count"

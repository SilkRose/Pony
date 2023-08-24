#!/usr/bin/env sh

flash_fiction_count=$(find "../flash-fiction/" -type f -name "*.md" | wc -l)

echo "$flash_fiction_count"

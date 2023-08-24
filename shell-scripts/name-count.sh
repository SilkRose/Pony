#!/usr/bin/env sh

name_count=$(grep -c "\- ." < "../stories/names.md")

echo "$name_count"

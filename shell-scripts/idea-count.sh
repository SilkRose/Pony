#!/usr/bin/env sh

idea_count=$(grep -c "^## " < "../stories/ideas.md")

echo "$idea_count"

#!/usr/bin/env sh

md_files=$(find "../" -type f -name "*.md" -not -name "*-meta.md" -not -name "ideas.md" -not -name "names.md" -not -name "README.md")

for file in $md_files; do
    word_count=$(tr -d "#>\-*–|—" < "$file" | sed '/<!--.*?-->/d' | wc -w)
    total_word_count=$((total_word_count + word_count))
done

echo "$total_word_count"

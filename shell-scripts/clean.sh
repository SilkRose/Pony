#!/usr/bin/env sh

md_files=$(find "../" -type f -name "*.md")

for file in $md_files; do
    sed -i \
        -e "s/[‘’\`´ʹ]/'/g" \
        -e 's/[“”‟″]/"/g' \
        -e 's/\.\.\./…/g' \
        -e 's/\,\*/\*,/g' \
        -e 's/\,_/_,/g' \
        -e 's/\-\-\-/—/g' \
        -e 's/\-\-/–/g' "$file"
done

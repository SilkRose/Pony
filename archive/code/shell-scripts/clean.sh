#!/usr/bin/env sh

set -o errexit
set -o nounset

md_files=$(find "../" -type f -name "*.md" -not -name "README.md")

for file in $md_files; do
	sed -i \
		-e "s/[‘’\`´ʹ]/'/g" \
		-e 's/[“”‟″]/"/g' \
		-e 's/\.\.\./…/g' \
		-e 's/\,\*/\*,/g' \
		-e 's/\,_/_,/g' \
		-e 's/\-\-\-/—/g' \
		-e 's/\-\-/–/g' "$file"

	if [ -z "${file##*_*}" ]; then
		new_path=$(echo "$file" | sed "s/_/-/g")
		mv "$file" "$new_path"
	fi
done

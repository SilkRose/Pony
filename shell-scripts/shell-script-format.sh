#!/usr/bin/env sh

if ! command -v shfmt > /dev/null 2>&1; then
    echo "shfmt is not installed. Please install it."
    exit 1
fi

find . -type f -name "*.sh" -exec shfmt -bn -ci -i 0 -p -sr -w {} \;

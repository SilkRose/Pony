#!/usr/bin/env sh

set -o errexit
set -o nounset

if ! command -v ruff > /dev/null 2>&1; then
	echo "ruff is not installed. Please install it."
	exit 1
fi

find . -type f -name "*.py" -exec ruff format {} \;

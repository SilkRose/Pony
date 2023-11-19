#!/usr/bin/env sh

set -o errexit
set -o nounset

if ! command -v pnpm > /dev/null 2>&1; then
	echo "pnpm is not installed. Please install it."
	exit 1
fi

if ! command -v rsync > /dev/null 2>&1; then
	echo "rsync is not installed. Please install it."
	exit 1
fi

pnpm install --no-lockfile

rsync --archive --copy-links ./node_modules/ ./node_modules_cp/
rm -r ./node_modules/
mv ./node_modules_cp/ ./node_modules/

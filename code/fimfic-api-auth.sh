#!/usr/bin/env sh

set -o errexit
set -o nounset

id=$1
secret=$2

curl --location "https://www.fimfiction.net/api/v2/token" \
	--data-urlencode "client_id=$id" \
	--data-urlencode "client_secret=$secret" \
	--data-urlencode "grant_type=client_credentials"

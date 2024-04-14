#!/usr/bin/env sh

set -o errexit
set -o nounset

id=$1
secret=$2
code=$3

curl --location "https://www.fimfiction.net/api/v2/token" \
	--data-urlencode "client_id=$id" \
	--data-urlencode "client_secret=$secret" \
	--data-urlencode "grant_type=authorization_code" \
	--data-urlencode "redirect_uri=https://fimfic-auth.silkrose.dev/" \
	--data-urlencode "code=$code"

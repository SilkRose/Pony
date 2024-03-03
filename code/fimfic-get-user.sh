#!/usr/bin/env sh

set -o errexit
set -o nounset

access_token=$1

curl --location --request GET \
	"https://www.fimfiction.net/api/v2/stories/551751/chapters" \
	--header "Authorization: Bearer $access_token" \
	--header "Content-Type: application/json" \

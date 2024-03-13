#!/usr/bin/env sh

set -o errexit
set -o nounset

read -r title
access_token=$1

json=$(jq -n \
	--arg title "$title" \
	'{
		"data": {
			"id": 1691611,
			"type": "chapter",
			"attributes": {
				"published": true
			}
		}
	}')

curl --location --request PATCH \
	"https://www.fimfiction.net/api/v2/chapters/1691611" \
	--header "Authorization: Bearer $access_token" \
	--header "Content-Type: application/json" \
	--data "$json"

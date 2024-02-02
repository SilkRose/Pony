#!/usr/bin/env sh

set -o errexit
set -o nounset

read -r bio
access_token=$1

json=$(jq -n \
	--arg bio "$bio" \
	'{
		"data": {
			"type": "user",
			"id": 237915,
			"attributes": {
				"bio": $bio
			}
		}
	}')

curl --location --request PATCH \
	"https://www.fimfiction.net/api/v2/users/237915" \
	--header "Authorization: Bearer $access_token" \
	--header "Content-Type: application/json" \
	--data "$json"

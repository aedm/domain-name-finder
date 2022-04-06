#!/usr/bin/env bash

#: ${AWS_REGION:?"Required: AWS_REGION"}
#: ${AWS_ACCESS_KEY_ID:?"Required: AWS_ACCESS_KEY_ID"}
#: ${AWS_SECRET_ACCESS_KEY:?"Required: AWS_SECRET_ACCESS_KEY"}

ACCESS_TOKEN="eyJraWQiOiJFczQ4dzhadTZISjhxd2F1M3M1bjJhMUUtSFN1Tk5PbW00Tl9oU0JwYW1RIiwiYWxnIjoiUlMyNTYifQ.eyJ2ZXIiOjEsImp0aSI6IkFULlJLMWtMSDI0ZUFQejRBb0JMQXNfRVZ6c1VMdmxZXzFuTmR4SHIyVjlEbHciLCJpc3MiOiJodHRwczovL2ljYW5uLWFjY291bnQub2t0YS5jb20vb2F1dGgyL2F1czJwMDFjMnJvSkFlQ2dZMnA3IiwiYXVkIjoiaHR0cDovL2FwaV9hdXRoZW5yaXphdGlvbl9zZXJ2ZXIuaWNhbm4ub3JnIiwiaWF0IjoxNjQ5MjQ1NDg2LCJleHAiOjE2NDkzMzE4ODYsImNpZCI6IjBvYTFyY2prcWtPbGlNUHVMMnA3IiwidWlkIjoiMDB1ZWhrNGw4a21oRzZVN2EycDciLCJzY3AiOlsib3BlbmlkIiwiaWNhbm4tY3VzdG9tIl0sImF1dGhfdGltZSI6MTY0OTI0NTQ4Niwic3ViIjoia29ydGV1ckBnbWFpbC5jb20iLCJnaXZlbl9uYW1lIjoiR8OhYm9yIiwiZmFtaWx5X25hbWUiOiJHeWVibsOhciIsImVtYWlsIjoia29ydGV1ckBnbWFpbC5jb20ifQ.kSaW6N3glxF3VPOD1am94SSqHLgmD0E4fK-Sd0xteGDL4sdCQWnLL2Ae2xXrSG1QwtLt8PTruaB6-vaTXPVMcfygPEA9QEYD1jTBpKDJZHG3k9t6gIw2CYKK42gzwyzDoa9lCQ746jTROOPbHNiPMGr3ocHddPUsLVV3aXqsDCVEPzQXamssOfXY2pHPxFm3_NWmAHMFHU1NR9ebx6T3tcUy-OLvPl0IryEphAhUzbc0jhwWTbGqscQgMg_BRYZoWcVI_9gBBKlmhru1EcB6UbuIrvBm80Du6oNHNAyGmPS09piHe2rDqCeQJuwr1DsR9fJuP4em2D-S-Tqg8wdKFQ"
ZONE_FILE_URL=https://czds-api.icann.org/czds/downloads/com.zone

# Check last update
FILE_HEADER=$(curl -I -H "Authorization: Bearer ${ACCESS_TOKEN}" ${ZONE_FILE_URL} | cat)
LAST_MODIFIED_RAW=$(echo "${FILE_HEADER}" | sed -n "s/^Last-Modified: \(.*\)$/\1/p")
LAST_MODIFIED=$(date -Iseconds -d "${LAST_MODIFIED_RAW}")
echo $LAST_MODIFIED

# Download file
curl -i -X GET -H "Authorization: Bearer ${ACCESS_TOKEN}" ${ZONE_FILE_URL} --output ./com.zone.txt.gz

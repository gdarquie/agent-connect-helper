#! /bin/sh

# upload a script to extract data in mongo on mongo server
docker cp -q mongo/extract-providers-fqdn.js fc_mongo-fca-low_1:/tmp/agent-connect-helper-extract-providers-fqdn.js

# only keep the last line and copy it in a file
OUTPUT=$(docker exec fc_mongo-fca-low_1 mongo -u rootAdmin -p pass --authenticationDatabase admin core-fca-low --tls tmp/agent-connect-helper-extract-providers-fqdn.js | tail -1)
mkdir -p data
echo "$OUTPUT" > data/local-fqdn-idp.json

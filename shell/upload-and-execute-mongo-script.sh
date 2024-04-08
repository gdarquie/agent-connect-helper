#! /bin/sh
FILENAME='update-fqdn'
# ARGS=$2

# upload a script to extract data in mongo on mongo server
docker cp -q mongo/$FILENAME.js fc_mongo-fca-low_1:/tmp/$FILENAME.js
docker cp -q mongo/$FILENAME.data.json fc_mongo-fca-low_1:/tmp/$FILENAME.data.json

# only keep the last line and copy it in a file
OUTPUT=$(docker exec fc_mongo-fca-low_1 mongo --eval "env='INTEGRATION', rollback=false, dryrun=false"  -u rootAdmin -p pass --authenticationDatabase admin core-fca-low --tls  tmp/$FILENAME.js)

echo "$OUTPUT"

set dotenv-load

post resource *args:
  curlie post -H "Authorization: ${MASTER_KEY}" 0.0.0.0:9001/{{resource}} {{args}}

put resource *args:
  curlie put -H "Authorization: ${MASTER_KEY}" 0.0.0.0:9001/{{resource}} {{args}}

get resource *args:
  curlie get -H "Authorization: ${MASTER_KEY}" 0.0.0.0:9001/{{resource}} {{args}}

del resource *args:
  curlie delete -H "Authorization: ${MASTER_KEY}" 0.0.0.0:9001/{{resource}} {{args}}

# Delete all records of the given resource
x resource:
  just api get {{resource}} --silent | jq -r '.[].id' | xargs -I {} just api del {{resource}}/{} --silent > /dev/null

# Update openapi.json
open:
  just api get openapi.json --silent | jq > {{justfile_directory()}}/docs/api-reference/openapi.json

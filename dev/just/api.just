post resource *args:
  curlie post localhost:9001/api/v1/{{resource}} {{args}}

put resource *args:
  curlie put localhost:9001/api/v1/{{resource}} {{args}}

get resource *args:
  curlie get localhost:9001/api/v1/{{resource}} {{args}}

del resource *args:
  curlie delete localhost:9001/api/v1/{{resource}} {{args}}

# Delete all records of the given resource
x resource:
  just api get {{resource}} --silent | jq -r '.[].id' | xargs -I {} just api del {{resource}}/{} --silent > /dev/null

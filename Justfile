mod db 'dev/just/db.just'
mod api 'dev/just/api.just'

# Create an ACME service.
seed:
  curl -X POST -H "Content-Type: application/json" --data-raw \
    '{ "name": "ACME", "redirect_url": "http://0.0.0.0:3000/", "logout_url": "http://0.0.0.0:3000/", "webhook_url": "http://0.0.0.0:3000/hook", "profile": { "fields": [ { "name": "First name", "required": true } ] } }' \
    --silent localhost:9001/services | jq --raw-output '.id' \
    | xargs -I {} just api post services/{}/keys kind=token key=$(openssl rand -hex 32) --silent

# Start all services
dev:
  dotenvx run -- devbox services up --process-compose-file {{justfile_directory()}}/dev/devbox/process-compose.yml

# Start the example service
example:
  cd {{justfile_directory()}}/example && cargo run

# Start docs preview
docs:
  cd {{justfile_directory()}}/docs && npx mintlify dev

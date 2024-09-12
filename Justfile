mod db 'dev/just/db.just'
mod api 'dev/just/api.just'

# Create an ACME service with a token key an the given webhook URL.
seed redirect_url webhook_url:
  curl -X POST -H "Content-Type: application/json" --data-raw \
    '{ "name": "ACME", "redirect_url": "{{redirect_url}}", "webhook_url": "{{webhook_url}}", "profile": { "fields": [ { "name": "First name", "required": true } ] } }' \
    --silent localhost:9001/services | jq --raw-output '.id' \
    | xargs -I {} just api post services/{}/keys kind=token key=$(openssl rand -hex 32) --silent

# Start all services
dev:
  dotenvx run -- devbox services up --process-compose-file {{justfile_directory()}}/dev/devbox/process-compose.yml

# Start docs preview
docs:
    cd {{justfile_directory()}}/docs && npx mintlify dev

set dotenv-load

mod db 'dev/just/db.just'
mod api 'dev/just/api.just'

# Create an ACME service.
seed local_ip_address:
  curlie post -H "Content-Type: application/json" -H "Authorization: ${MASTER_KEY}" --data-raw \
    '{ "name": "ACME", "redirect_url": "http://{{local_ip_address}}:3000/", "logout_url": "http://{{local_ip_address}}:3000/", "webhook_url": "http://{{local_ip_address}}:3000/hook", "profile": { "fields": [ { "name": "First name", "required": true } ] } }' \
    {{local_ip_address}}:9001/services

# Start all Mist services.
dev:
  devbox services up --process-compose-file {{justfile_directory()}}/dev/devbox/process-compose.yml

# Start demo service.
demo:
  cd {{justfile_directory()}}/demo && cargo watch -x run

# Start docs preview.
docs:
  cd {{justfile_directory()}}/docs && npx mintlify dev

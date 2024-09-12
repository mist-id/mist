_lint *args:
  pre-commit run sqlfluff-fix --hook-stage manual {{args}}

# Lint staged files with sqlfluff
lint:
  just db lint

# Lint all files with sqlfluff
lint-all:
  just db lint --all-files

# Run migrations
migrate:
  @ dotenvx run -- sqlx migrate run --source {{justfile_directory()}}/db/migrations
  @ echo "👻 Migrations complete!" | gum format

# Export query matadata
prepare:
  @ cd {{justfile_directory()}}/db && cargo sqlx prepare

# Dump the schema to `db/schema.sql`
dump:
  @ pg_dump --dbname=mist --schema-only --file {{justfile_directory()}}/db/schema.sql
  @ echo "👻 Schema dumped to \`db/schema.sql\`" | gum format

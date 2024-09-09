#!/bin/sh
DB=mist
USER=casper

if ! psql -lqt | cut -d \| -f 1 | grep -qw $DB; then
  echo "[Postgres] Creating $DB..."

  createdb $DB
fi

if ! psql -d $DB -c "SELECT 1 FROM pg_roles WHERE rolname='$USER'" | grep -q 1; then
  echo "[Postgres] Summoning $USER..."

  createuser -s $USER
fi

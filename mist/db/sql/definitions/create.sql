insert into definitions (service_id, name, value, is_default) values ($1, $2, $3, $4) returning
  id, name, is_default, value as "value: _", service_id, created_at, updated_at;

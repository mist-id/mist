insert into keys (service_id, kind, value, priority) values ($1, $2, $3, $4) returning
  id, service_id, kind as "kind: _", value, priority, is_active, created_at, updated_at;

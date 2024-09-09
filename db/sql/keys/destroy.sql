delete from keys where id = $1 returning
  id, service_id, kind as "kind: _", value, priority, is_active, created_at, updated_at;

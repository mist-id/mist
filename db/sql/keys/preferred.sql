select id, service_id, kind as "kind: _", value, priority, is_active, created_at, updated_at
  from keys where service_id = $1 and kind = $2 and is_active = true order by priority asc limit 1;

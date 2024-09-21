select id, name, is_default, value as "value: _", service_id, created_at, updated_at
  from definitions where service_id = $1 and is_default = true;

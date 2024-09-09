update keys set priority = priority + 1 where service_id = $1 and kind = $2 and is_active = true;

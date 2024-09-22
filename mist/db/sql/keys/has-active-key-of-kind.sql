select case
  when count(*) > 1 then true else false
end as has_active_keys
from keys where service_id = $1 and kind = $2 and is_active = true;

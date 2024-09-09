update services set name = $2, redirect_url = $3, webhook_url = $4 where id = $1 returning *;

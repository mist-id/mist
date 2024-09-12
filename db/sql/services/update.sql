update services set name = $2, redirect_url = $3, logout_url = $4, webhook_url = $5 where id = $1 returning *;

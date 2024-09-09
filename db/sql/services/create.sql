insert into services (name, redirect_url, webhook_url) values ($1, $2, $3) returning *;

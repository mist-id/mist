insert into services (name, redirect_url, logout_url, webhook_url) values ($1, $2, $3, $4) returning *;

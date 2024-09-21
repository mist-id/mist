-- Add up migration script here
create or replace function set_updated_at() returns trigger as $$ begin
    new.updated_at = now();
    return new;
end; $$ language plpgsql;

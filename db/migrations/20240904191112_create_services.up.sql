-- Add up migration script here
create table services (
    id uuid primary key default uuid_generate_v4(),
    name text unique not null,
    redirect_url text not null,
    webhook_url text not null,

    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

create or replace trigger set_updated_at before update on services
for each row execute function set_updated_at();

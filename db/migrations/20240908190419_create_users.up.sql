-- Add up migration script here
create table users (
    id uuid primary key default uuid_generate_v4(),
    service_id uuid not null references services(id) on delete cascade,

    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

create or replace trigger set_updated_at before update on users
for each row execute function set_updated_at();

-- Add up migration script here
create table definitions (
    id uuid primary key default uuid_generate_v4(),
    name text not null,
    value jsonb not null,
    is_default boolean not null default false,
    service_id uuid not null references services(id) on delete cascade,

    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now(),

    unique (service_id, is_default)
);

create or replace trigger set_updated_at before update on definitions
for each row execute function set_updated_at();

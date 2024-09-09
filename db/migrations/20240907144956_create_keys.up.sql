-- Add up migration script here
create type key_kind as enum ('token');

create table keys (
    id uuid primary key default uuid_generate_v4(),
    kind key_kind not null,
    value bytea not null,
    priority int not null,
    is_active boolean not null default true,
    service_id uuid not null references services(id) on delete cascade,

    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now(),

    unique (service_id, kind, priority)
);

create or replace trigger set_updated_at before update on keys
for each row execute function set_updated_at();

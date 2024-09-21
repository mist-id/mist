-- Add up migration script here
create table identifiers (
    id uuid primary key default uuid_generate_v4(),
    value text unique not null,
    user_id uuid not null references users(id) on delete cascade,

    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

create or replace trigger set_updated_at before update on identifiers
for each row execute function set_updated_at();

-- Add down migration script here
drop trigger if exists set_updated_at on keys;
drop table if exists keys;
drop type if exists key_kind;

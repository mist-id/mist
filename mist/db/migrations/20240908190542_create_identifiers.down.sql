-- Add down migration script here
drop trigger if exists set_updated_at on identifiers;
drop table if exists identifiers;

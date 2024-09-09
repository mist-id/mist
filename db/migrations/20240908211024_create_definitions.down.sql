-- Add down migration script here
drop trigger if exists set_updated_at on definitions;
drop table if exists definitions;

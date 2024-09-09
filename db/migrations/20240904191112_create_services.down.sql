-- Add down migration script here
drop trigger if exists set_updated_at on services;
drop table if exists services;

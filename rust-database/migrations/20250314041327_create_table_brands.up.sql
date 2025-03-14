-- Add up migration script here
CREATE TABLE brands(
  id serial primary key,
	name varchar(100) not null,
	description text,
	created_at timestamp not null default current_timestamp,
	updated_at timestamp not null default current_timestamp
);
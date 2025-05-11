create type gender as enum ('male', 'female');

create table if not exists "user" (
	   id int primary key generated always as identity,
	   nick varchar(32) not null unique,
	   gender gender not null,
	   password text not null,
	   create_at timestamptz not null default now()
);

create table if not exists "session" (
	   id int primary key generated always as identity,
	   token uuid not null unique,
	   user_id int not null references "user" ("id"),
	   create_at timestamptz not null default now()
);

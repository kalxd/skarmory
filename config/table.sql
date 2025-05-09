create table if not exists "user" (
	   id int primary key generated always as identity,
	   nickname varchar(32) not null unique,
	   password text not null,
	   create_at timestamptz not null default now()
);

create table if not exists "session" (
	   id int primary key generated always as identity,
	   token uuid not null unique,
	   user_id int not null references "user" ("id"),
	   create_at timestamptz not null default now()
);

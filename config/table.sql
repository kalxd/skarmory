create type gender as enum ('male', 'female');

create type cup_op as enum ('pour-in', 'pour-out', 'drink');

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

create table if not exists "cup" (
	   id int primary key generated always as identity,
	   user_id int not null references "user" ("id"),
	   nick varchar(64),
	   volum int not null,
	   color varchar(32) not null default '白色',
	   create_at timestamptz not null default now()
);

create table if not exists "cup_operator" (
	   id int primary key generated always as identity,
	   cup_id int not null references "cup" ("id"),
	   op cup_op not null,
	   value int not null,
	   create_at timestamptz not null default now()
);

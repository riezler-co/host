-- Your SQL goes here

create table if not exists sites
	( id uuid primary key default uuid_generate_v4()
	, name text not null
	, slug text not null unique
	, updated_at timestamptz not null default now()
	, created_at timestamptz not null default now()
	);

create index if not exists site_slug on sites(slug);
select diesel_manage_updated_at('sites');
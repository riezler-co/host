-- Your SQL goes here

create table if not exists branches
	( id uuid primary key default uuid_generate_v4()
	, slug text not null
	, name text not null
	, version smallint not null
	, site_id uuid not null references sites(id) on delete cascade
	, is_public boolean not null default false 
	, created_at timestamptz not null default now()
	, updated_at timestamptz not null default now()
	, unique(slug, site_id)
	, unique(site_id, name)
	);

create index if not exists branch_slug on branches(slug);
create index if not exists branch_site on branches(site_id);

select diesel_manage_updated_at('branches');
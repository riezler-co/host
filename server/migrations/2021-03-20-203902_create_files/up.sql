-- Your SQL goes here

create table if not exists files
	( id uuid primary key default uuid_generate_v4()
	, path text not null
	, content bytea not null
	, deployment_id uuid not null references deployments(id) on delete cascade
	, size integer not null
	, extension text not null
	, unique(deployment_id, path)
	, created_at timestamptz not null default now()
	)
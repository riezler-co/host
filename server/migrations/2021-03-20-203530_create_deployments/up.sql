-- Your SQL goes here

create table if not exists deployments
	( id uuid primary key default uuid_generate_v4()
	, version smallint not null
	, branch_id uuid not null references branches(id) on delete cascade
	, config jsonb
	, created_at timestamptz not null default now()
	, status text not null
	, unique(version, branch_id)
	);

create index if not exists deployment_branch on deployments(branch_id);
create index if not exists deployment_version on deployments(version);

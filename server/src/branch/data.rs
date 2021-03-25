use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::PgPool;
use types::NewBranch;
use uuid::Uuid;

#[derive(Serialize)]
pub struct Branch {
    pub id: Uuid,
    pub slug: String,
    pub name: String,
    pub version: i16,
    pub site_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_public: bool,
}

impl Branch {
    pub async fn list(pool: &PgPool, site: &Uuid) -> Result<Vec<Branch>, sqlx::Error> {
        sqlx::query_as!(
            Branch,
            r#"
				select id, slug, name, version, site_id, created_at, updated_at, is_public
				  from branches
				 where site_id = $1
				 order by name asc 
			"#,
            site,
        )
        .fetch_all(pool)
        .await
    }

    pub async fn create(
        pool: &PgPool,
        site: &Uuid,
        branch: NewBranch,
    ) -> Result<Branch, sqlx::Error> {
        sqlx::query_as!(
            Branch,
            r#"
    			insert into branches(site_id, name, slug, version)
    			values($1, $2, $3, 1)
    			returning id, slug, name, version, site_id, created_at, updated_at, is_public
    		"#,
            site,
            branch.name,
            branch.slug
        )
        .fetch_one(pool)
        .await
    }

    pub async fn create_deployment(
        pool: &PgPool,
        site: &String,
        branch: NewBranch,
    ) -> Result<Branch, sqlx::Error> {
        sqlx::query_as!(
            Branch,
            r#"
                with site_info as (
                    select id
                      from sites
                     where slug = $1 
                )
                insert into branches(site_id, name, slug, version)
                select site_info.id as site_id
                     , $2 as name
                     , $3 as slug
                     , 1 as version
                  from site_info
                on conflict (slug, site_id)
                         do update set slug = excluded.slug
                returning id, slug, name, version, site_id, created_at, updated_at, is_public
            "#,
            site,
            branch.name,
            branch.slug
        )
        .fetch_one(pool)
        .await
    }

    pub async fn update(pool: &PgPool, id: &Uuid, branch: NewBranch) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
    			update branches
    			   set name = $2
    			     , slug = $3
    			 where id = $1
    		"#,
            id,
            branch.name,
            branch.slug,
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn get(pool: &PgPool, id: &Uuid) -> Result<Option<Branch>, sqlx::Error> {
        sqlx::query_as!(
            Branch,
            r#"
    			select id, slug, name, version, site_id, created_at, updated_at, is_public
    			  from branches
    			 where id = $1
    		"#,
            id,
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, id: &Uuid) -> Result<(), sqlx::Error> {
        sqlx::query_as!(
            Branch,
            r#"
    			delete from branches
    			 where id = $1
    		"#,
            id,
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn set_visibility(
        pool: &PgPool,
        id: &Uuid,
        visibility: bool,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
    			update branches
    			   set is_public = $2
    			 where id = $1 
    		"#,
            id,
            visibility
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn set_version(pool: &PgPool, id: &Uuid, version: i16) -> Result<i16, sqlx::Error> {
        sqlx::query!(
            r#"
    			with max_version as (
    				select max(deployments.version) as version
    				  from deployments
    				 where deployments.branch_id = $1
    			)
    			update branches
    			   set version = least($2, max_version.version)
    			  from max_version
    			 where branches.id = $1
    			returning branches.version
    		"#,
            id,
            version,
        )
        .fetch_one(pool)
        .await
        .map(|row| row.version)
    }
}

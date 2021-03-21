use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct NewSite {
    name: String,
    slug: String,
}

#[derive(Serialize)]
pub struct Site {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl Site {
    pub async fn get(pool: &PgPool, slug: &Uuid) -> Result<Option<Site>, sqlx::Error> {
        sqlx::query_as!(
            Site,
            r#"
				select id, name, slug, updated_at, created_at
				  from sites
				 where id = $1
			"#,
            slug,
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn create(pool: &PgPool, site: NewSite) -> Result<Site, sqlx::Error> {
        sqlx::query_as!(
            Site,
            r#"
    			insert into sites(name, slug)
    			values($1, $2)
    			returning id, name, slug, updated_at, created_at
    		"#,
            site.name,
            site.slug,
        )
        .fetch_one(pool)
        .await
    }

    pub async fn update(pool: &PgPool, id: &Uuid, site: NewSite) -> Result<Site, sqlx::Error> {
        sqlx::query_as!(
            Site,
            r#"
    			update sites
    			   set name = $2
    			     , slug = $3
    			 where id = $1  
    			returning id, name, slug, updated_at, created_at
    		"#,
            id,
            site.name,
            site.slug,
        )
        .fetch_one(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, id: &Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
    			delete from sites
    			 where id = $1
    		"#,
            id,
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn list(pool: &PgPool) -> Result<Vec<Site>, sqlx::Error> {
        sqlx::query_as!(
            Site,
            r#"
    			select id, name, slug, updated_at, created_at
    			  from sites
    			 order by name asc
    		"#
        )
        .fetch_all(pool)
        .await
    }
}

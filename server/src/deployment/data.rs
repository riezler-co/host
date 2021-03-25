use crate::branch::data::Branch;
use types::DeploymentConfig;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json;
use serde_plain;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub enum DeploymentStatus {
    #[serde(rename = "in_progress")]
    InProgress,

    #[serde(rename = "complete")]
    Complete,

    #[serde(rename = "abort")]
    Abort,
}

#[derive(Serialize, Deserialize)]
pub struct Deployment {
    pub id: Uuid,
    pub version: i16,
    pub branch_id: Uuid,
    pub config: DeploymentConfig,
    pub created_at: DateTime<Utc>,
    pub status: DeploymentStatus,
}

#[derive(Serialize)]
pub struct PartialDeployment {
    pub id: Uuid,
    pub version: i16,
    pub config: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub status: String,
}

impl Deployment {
    pub async fn create(
        pool: &PgPool,
        branch: &Uuid,
        config: DeploymentConfig,
    ) -> Result<Deployment, sqlx::Error> {
        let config_json = serde_json::to_value(&config).unwrap();
        let status = serde_plain::to_string(&DeploymentStatus::InProgress).unwrap();

        let row = sqlx::query!(
            r#"
				with next_version as (
					select greatest(deployments.version, 0) + 1 as version
                         , branches.id as branch_id 
                      from branches
                      left join deployments on deployments.branch_id = branches.id
                     where branches.id = $1
                     order by version desc
                     limit 1
				)
				insert into deployments(version, branch_id, config, status)
				select next_version.version
					 , $1 as branch_id
					 , $2 as config
					 , $3 as status
				  from next_version
				returning id, version, branch_id, created_at
			"#,
            branch,
            config_json,
            status,
        )
        .fetch_one(pool)
        .await?;

        Ok(Deployment {
            config,
            id: row.id,
            version: row.version,
            branch_id: row.branch_id,
            created_at: row.created_at,
            status: DeploymentStatus::InProgress,
        })
    }

    pub async fn delete(pool: &PgPool, id: &Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
    			delete from deployments
    			 where id = $1 
    		"#,
            id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn abort(pool: &PgPool, id: &Uuid) -> Result<(), sqlx::Error> {
        let status = serde_plain::to_string(&DeploymentStatus::Abort).unwrap();
        sqlx::query!(
            r#"
    			update deployments
    			   set status = $2
    			 where id = $1  
    		"#,
            id,
            status,
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn complete(pool: &PgPool, id: &Uuid) -> Result<Branch, sqlx::Error> {
        let status = serde_plain::to_string(&DeploymentStatus::Complete).unwrap();
        sqlx::query_as!(
            Branch,
            r#"
            	with set_complete as (
	    			update deployments
	    			   set status = $2
	    			 where id = $1
	    			returning branch_id, version
            	)
            	update branches
            	   set version = set_complete.version
            	  from set_complete
            	 where branches.id = set_complete.branch_id
            	returning branches.id
            			, branches.slug
            			, branches.name
            			, branches.version
            			, branches.site_id
            			, branches.created_at
            			, branches.updated_at
            			, branches.is_public
    		"#,
            id,
            status,
        )
        .fetch_one(pool)
        .await
    }

    pub async fn list(pool: &PgPool, branch: &Uuid) -> Result<Vec<PartialDeployment>, sqlx::Error> {
        sqlx::query_as!(
            PartialDeployment,
            r#"
    			select id
					 , version
					 , config
					 , created_at
					 , status
				  from deployments
				 where branch_id = $1
    		"#,
            branch,
        )
        .fetch_all(pool)
        .await
    }

    pub async fn get(pool: &PgPool, id: &Uuid) -> Result<Option<PartialDeployment>, sqlx::Error> {
        sqlx::query_as!(
            PartialDeployment,
            r#"
    			select id
					 , version
					 , config
					 , created_at
					 , status
				  from deployments
				 where id = $1
    		"#,
            id
        )
        .fetch_optional(pool)
        .await
    }
}

use chrono::{DateTime, Utc};
use rocket::http::{ContentType, Header};
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use serde::Serialize;
use sqlx::PgPool;
use std::io::Cursor;
use types::NewFile;
use uuid::Uuid;

#[derive(Serialize)]
pub struct File {
    pub id: Uuid,
    pub path: String,
    pub content: Vec<u8>,
    pub size: i32,
    pub extension: String,
    pub created_at: DateTime<Utc>,
}

impl File {
    pub async fn create(
        pool: &PgPool,
        deployment: &Uuid,
        file: NewFile,
    ) -> Result<Uuid, sqlx::Error> {
        sqlx::query!(
            r#"
				insert into files(deployment_id, path, content, size, extension)
				values($1, $2, $3, $4, $5)
				returning id
			"#,
            deployment,
            file.path,
            file.content,
            file.size,
            file.extension
        )
        .fetch_one(pool)
        .await
        .map(|row| row.id)
    }

    pub async fn get(pool: &PgPool, file: &Uuid) -> Result<Option<File>, sqlx::Error> {
        sqlx::query_as!(
            File,
            r#"
    			select id, path, content, size, extension, created_at
    			  from files
    			 where id = $1
    		"#,
            file
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn serve(
        pool: &PgPool,
        site: &str,
        branch: &str,
        path: &str,
    ) -> Result<Option<File>, sqlx::Error> {
        sqlx::query_as!(
            File,
            r#"
    			select files.id
    				 , files.path
    				 , files.content
    				 , files.size
    				 , files.extension
    				 , files.created_at
    			  from sites

    			  join branches on branches.site_id = sites.id

    			  join deployments on deployments.branch_id = branches.id
    			                  and deployments.version = branches.version

    			  join files on files.deployment_id = deployments.id

    			 where sites.slug = $1
    			   and branches.slug = $2
    			   and files.path = $3
    		"#,
            site,
            branch,
            path
        )
        .fetch_optional(pool)
        .await
    }
}

impl<'r> Responder<'r, 'static> for File {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let content_type = match ContentType::from_extension(&self.extension) {
            Some(ct) => ct,
            None => ContentType::new("text", "plain"),
        };

        let last_modified = self.created_at.to_string();

        Response::build()
            .sized_body(self.content.len(), Cursor::new(self.content))
            .header(content_type)
            .header(Header::new("Last-Modified", last_modified))
            .ok()
    }
}

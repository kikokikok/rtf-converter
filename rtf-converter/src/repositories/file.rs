use crate::infra::db::postgres::RtfDb;
use crate::models::file::{NewFile, File, FileConditions, FileIdentifier, FileList};
use anyhow::{Result, Context};
use async_trait::async_trait;
use uuid::Uuid;
use validator::Validate;

#[cfg(test)]
use mockall::{automock, mock, predicate::*};

#[cfg_attr(test, automock)]
#[async_trait]
pub trait FileRepo {
	async fn find_all(&self, conditions: &FileConditions) -> Result<FileList>;
	async fn add(&self, file_data: &NewFile) -> Result<FileIdentifier>;
	async fn find_by_id(&self, file_id: Uuid) -> Result<File>;
	async fn delete(&self, file_id: Uuid) -> Result<()>;
}

pub struct FileRepoImpl {
	pool: RtfDb,
}

impl FileRepoImpl {
	pub fn new(pool: RtfDb) -> Self {
		Self { pool }
	}
}

#[async_trait]
impl FileRepo for FileRepoImpl {

	async fn find_all(&self, conditions: &FileConditions) -> Result<FileList> {
		let mut query = sqlx::query_as::<_, File>("select * from files");
		if let Some(name) = &conditions.file_name {
			query = sqlx::query_as::<_, File>("select * from files where file_name LIKE $1")
				.bind(format!("%{}%", name))
		}
		let result = query
			.fetch_all(&*self.pool)
			.await
			.context("DB ERROR (find all files)")?;
		Ok(result)
	}

	async fn add(&self, file_data: &NewFile) -> Result<FileIdentifier> {
		if let Err(errors) = file_data.validate() {
			let err_messages: Vec<String> = errors
				.field_errors()
				.into_iter()
				.map(|(field, error)| format!("{}: {:?}", field, error))
				.collect();
		}

		let row = sqlx::query_as::<_, FileIdentifier>(
			r#"
			INSERT INTO files (tenant_id, owner_id, file_binary_content, content_type, file_name, max_age, templating_engine, templating_engine_version, version)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING id, version
            "#,
		)
			.bind(&file_data.tenant_id)
			.bind(&file_data.owner_id)
			.bind(&file_data.file_binary_content)
			.bind(&file_data.content_type)
			.bind(&file_data.file_name)
			.bind(&file_data.max_age)
			.bind(&file_data.templating_engine)
			.bind(&file_data.templating_engine_version)
			.bind(1)
			.fetch_one(&*self.pool)
			.await
			.context("DB ERROR (create file)")?;
		Ok(row)
	}

	async fn find_by_id(&self, file_id: Uuid) -> Result<File> {
		let row = sqlx::query_as::<_, File>("select * from users where id = $1")
			.bind(file_id)
			.fetch_one(&*self.pool)
			.await
			.context("DB ERROR (find file by id)")?;
		Ok(row)
	}

	async fn delete(&self, file_id: Uuid) -> Result<()> {
		sqlx::query("DELETE FROM files WHERE id = $1")
			.execute(&*self.pool)
			.await?;

		Ok(())
	}
}

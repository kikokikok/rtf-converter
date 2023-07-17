use sqlx::{types::Uuid, PgPool};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(sqlx::FromRow, Serialize, Deserialize, Validate)]
pub struct File {
	#[serde(skip_serializing)]
	pub id: Uuid,

	#[serde(skip_serializing)]
	pub tenant_id: Uuid,

	#[serde(skip_serializing)]
	pub owner_id: Option<Uuid>,

	pub file_binary_content: Vec<u8>,

	#[validate(length(min = 1, max = 255))]
	pub content_type: String,

	#[validate(length(min = 1, max = 255))]
	pub file_name: String,

	#[validate(range(min = 0))]
	pub file_size: u64,

	#[serde(skip_serializing)]
	pub insertion_date: chrono::NaiveDateTime,

	#[validate]
	pub max_age: Option<chrono::Duration>,

	pub templating_engine: Option<String>,

	pub templating_engine_version: Option<String>,

	#[validate(range(min = 1))]
	pub version: i32,
}

impl File {
	pub async fn create(file: &File, pool: &PgPool) -> Result<(), sqlx::Error> {
		if let Err(errors) = file.validate() {
			let err_messages: Vec<String> = errors
				.field_errors()
				.into_iter()
				.map(|(field, error)| format!("{}: {:?}", field, error))
				.collect();

			return Err(sqlx::Error::Decode(
				Box::new(sqlx::Error::RowNotFound), // Use a custom error type if desired
			));
		}

		sqlx::query!(
            "INSERT INTO files (id, tenant_id, owner_id, file_binary_content, content_type, file_name, file_size, insertion_date, max_age, templating_engine, templating_engine_version, version)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)",
            file.id,
            file.tenant_id,
            file.owner_id,
            file.file_binary_content,
            file.content_type,
            file.file_name,
            file.file_size,
            Utc::now().naive_utc(),
            file.max_age,
            file.templating_engine,
            file.templating_engine_version,
            file.version
        )
			.execute(pool)
			.await?;

		Ok(())
	}

	pub async fn read(id: Uuid, pool: &PgPool) -> Result<Option<File>, sqlx::Error> {
		let file = sqlx::query_as!(
            File,
            "SELECT * FROM files WHERE id = $1",
            id
        )
			.fetch_optional(pool)
			.await?;

		Ok(file)
	}

	pub async fn update(file: &File, pool: &PgPool) -> Result<(), sqlx::Error> {
		if let Err(errors) = file.validate() {
			let err_messages: Vec<String> = errors
				.field_errors()
				.into_iter()
				.map(|(field, error)| format!("{}: {:?}", field, error))
				.collect();

			return Err(sqlx::Error::Decode(
				Box::new(sqlx::Error::RowNotFound), // Use a custom error type if desired
			));
		}

		sqlx::query!(
            "UPDATE files SET tenant_id = $1, owner_id = $2, file_binary_content = $3, content_type = $4, file_name = $5, file_size = $6, insertion_date = $7, max_age = $8, templating_engine = $9, templating_engine_version = $10, version = $11 WHERE id = $12",
            file.tenant_id,
            file.owner_id,
            file.file_binary_content,
            file.content_type,
            file.file_name,
            file.file_size,
            file.insertion_date,
            file.max_age,
            file.templating_engine,
            file.templating_engine_version,
            file.version,
            file.id
        )
			.execute(pool)
			.await?;

		Ok(())
	}

	pub async fn delete(id: Uuid, pool: &PgPool) -> Result<(), sqlx::Error> {
		sqlx::query!("DELETE FROM files WHERE id = $1", id)
			.execute(pool)
			.await?;

		Ok(())
	}
}

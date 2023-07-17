use crate::infra::db::postgres::Db;
use crate::error::Result;
use crate::models::user::{NewUser, User, UserConditions, UserId, UserList};
use anyhow::Context;
use async_trait::async_trait;
use mockall::automock;
use crate::models::file::{File, FileId};

pub struct FileRepoImpl {
	pool: Db,
}
impl UserRepoImpl {
	pub fn new(pool: Db) -> Self {
		Self { pool: pool }
	}
}

#[automock]
#[async_trait]
pub trait UserRepo {
	async fn find_all(&self, conditions: &UserConditions) -> Result<UserList>;
	async fn add(&self, user_data: &NewUser) -> Result<UserId>;
	async fn find_by_id(&self, user_id: i32) -> Result<User>;
}

#[async_trait]
impl UserRepo for FileRepoImpl {
	async fn find_all(&self, conditions: &UserConditions) -> Result<FileList> {
		let mut query = sqlx::query_as::<_, File>("select * from files");
		if let Some(name) = &conditions.name {
			query = sqlx::query_as::<_, File>("select * from files where name LIKE $1")
				.bind(format!("%{}%", name))
		}
		let result = query
			.fetch_all(&*self.pool)
			.await
			.context("DB ERROR (find all users)")?;
		Ok(result)
	}

	async fn add(&self, file_data: &NewFile) -> Result<FileId> {
		let row = sqlx::query_as::<_, FileId>(
			r#"
			INSERT INTO files (tenant_id, owner_id, file_binary_content, content_type, file_name, max_age, templating_engine, templating_engine_version, version)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING id
            "#,
		)
			.bind(&file_data.name)
			.bind(&file_data.msg)
			.bind(&file_data.age)
			.fetch_one(&*self.pool)
			.await
			.context("DB ERROR (create file)")?;
		Ok(row)
	}

	async fn find_by_id(&self, file_id: i32) -> Result<File> {
		let row = sqlx::query_as::<_, File>("select * from users where id = $1")
			.bind(file_id)
			.fetch_one(&*self.pool)
			.await
			.context("DB ERROR (find file by id)")?;
		Ok(row)
	}
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

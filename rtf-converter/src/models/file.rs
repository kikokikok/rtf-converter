use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};
use sqlx::FromRow;

pub type UserList = Vec<File>;

#[derive(Serialize, Deserialize, Debug)]
pub struct NewFile {
	pub name: String,
	pub msg: Option<String>,
	pub age: Option<i16>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileConditions {
	pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, FromRow, Eq)]
pub struct FileId {
	pub id: Uuid,
	pub tenant_id: Uuid,
	pub version: i32,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Validate, Debug)]
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
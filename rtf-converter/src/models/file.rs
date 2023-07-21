use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use validator::{Validate};
use sqlx::{FromRow, Type};
use uuid::Uuid;

pub type FileList = Vec<File>;

#[serde_as]
#[derive(FromRow, Serialize, Deserialize, Debug, Validate)]
pub struct NewFile {
	pub tenant_id: Option<Uuid>,
	pub owner_id: Option<Uuid>,
	pub file_binary_content: Vec<u8>,
	#[validate(length(min = 1, max = 255))]
	pub content_type: String,
	#[validate(length(min = 1, max = 255))]
	pub file_name: String,
	#[validate(range(min = 0))]
	pub file_size: i64,
	pub insertion_date: Option<chrono::DateTime<chrono::Utc>>,
	pub max_age: Option<chrono::DateTime<chrono::Utc>>,
	pub templating_engine: Option<String>,
	pub templating_engine_version: Option<String>,
}

#[derive(FromRow, Type, Serialize, Deserialize, Debug)]
pub struct FileConditions {
	pub file_name: Option<String>,
}

#[derive(FromRow, Type, Serialize, Deserialize, Debug, PartialEq, Eq, Validate)]
pub struct FileIdentifier {
	pub unique_id: Uuid,
	#[validate(range(min = 1))]
	pub version: i32,
}

#[serde_as]
#[derive(FromRow, Serialize, Deserialize, Validate, Debug)]
pub struct File {
	pub id:  FileIdentifier,
	pub tenant_id: Uuid,
	pub owner_id: Option<Uuid>,
	pub file_binary_content: Vec<u8>,
	#[validate(length(min = 1, max = 255))]
	pub content_type: String,
	#[validate(length(min = 1, max = 255))]
	pub file_name: String,
	#[validate(range(min = 0))]
	pub file_size: i64,
	pub insertion_date: Option<chrono::DateTime<chrono::Utc>>,
	pub max_age: Option<chrono::DateTime<chrono::Utc>>,
	pub templating_engine: Option<String>,
	pub templating_engine_version: Option<String>,
}
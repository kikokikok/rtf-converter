use crate::infra::db::postgres;
use crate::repositories::{
	file::{FileRepo, FileRepoImpl},
};
use axum::extract::Extension;
use std::sync::Arc;

pub mod file;

pub type RepoExt = Extension<Arc<RepoImpls>>;

pub async fn create_repositories() -> RepoImpls {
	let rtf_db_pool = Arc::new(postgres::rtf_db_connect().await);
	RepoImpls::new(
		FileRepoImpl::new(rtf_db_pool.clone()),
	)
}

pub struct RepoImpls {
	pub file: FileRepoImpl,
}
impl RepoImpls {
	pub fn new(
		file_repo_impl: FileRepoImpl,
	) -> Self {
		Self {
			file: file_repo_impl,
		}
	}
}

pub trait Repositories {
	type FileRepoImpl: FileRepo;

	fn file(&self) -> &Self::FileRepoImpl;
}
impl Repositories for RepoImpls {
	type FileRepoImpl = FileRepoImpl;

	fn file(&self) -> &Self::FileRepoImpl {
		&self.file
	}
}
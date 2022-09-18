mod client;
pub(crate) use client::GitHubClient;

mod model;

mod repository;
pub use repository::GitHubRepositoryImpl;

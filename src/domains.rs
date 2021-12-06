pub mod custom_error;
pub mod redmine;
pub mod redmine_api;
mod redmine_client;
pub mod status;

cfg_if::cfg_if! {
    if #[cfg(test)] {
        pub use redmine_client::MockRedmineClient as RedmineClient;
    } else {
        pub use redmine_client::RedmineClient;
    }
}

pub mod commands;
pub mod domains;
pub mod globals;
pub mod listeners;
pub mod utils;

#[cfg(test)]
mod test {
    use crate::domains::redmine;

    #[tokio::test]
    async fn fetch_issue() {
        match redmine::fetch_record_issue(9690).await {
            Ok(issue) => {
                // TODO test
                println!("{:#?}", issue.relations);
            },
            Err(err) => {
                println!("{}", err);
            }
        }
    }
}

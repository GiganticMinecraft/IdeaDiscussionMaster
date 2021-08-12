pub mod commands;
pub mod domains;
pub mod globals;
pub mod listeners;
pub mod utils;

#[cfg(test)]
mod test {
    use std::str::FromStr;
    use test_case::test_case;

    use crate::{domains::redmine, globals::agendas::AgendaStatus};

    #[test_case("new" => AgendaStatus::New; "newから(insensitive)")]
    #[test_case("New" => AgendaStatus::New; "Newから")]
    #[test_case("approved" => AgendaStatus::Approved; "Approvedから(insensitive)")]
    #[test_case("Approved" => AgendaStatus::Approved; "Approvedから")]
    #[test_case("declined" => AgendaStatus::Declined; "Declinedから(insensitive)")]
    #[test_case("Declined" => AgendaStatus::Declined; "Declinedから")]
    fn agenda_status_from_str(str: &str) -> AgendaStatus {
        AgendaStatus::from_str(str).unwrap()
    }

    #[test_case(AgendaStatus::New => "🆕")]
    #[test_case(AgendaStatus::Approved => "⭕")]
    #[test_case(AgendaStatus::Declined => "❌")]
    fn agenda_status_to_emoji(status: AgendaStatus) -> String {
        status.emoji()
    }

    #[test_case(AgendaStatus::New => "新規")]
    #[test_case(AgendaStatus::Approved => "承認")]
    #[test_case(AgendaStatus::Declined => "却下")]
    fn agenda_status_to_ja(status: AgendaStatus) -> String {
        status.ja()
    }

    #[test]
    fn agenda_statuses_can_be_done() {
        assert_eq!(AgendaStatus::done_statuses(), vec!(AgendaStatus::Approved, AgendaStatus::Declined));
    }

    #[tokio::test]
    #[ignore]
    async fn fetch_issue() {
        match redmine::fetch_record_issue(9690).await {
            Ok(issue) => {
                // TODO: test
                println!("{:#?}", issue.relations);
            }
            Err(err) => {
                println!("{}", err);
            }
        }
    }
}

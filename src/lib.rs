pub mod commands;
pub mod domains;
pub mod globals;
pub mod listeners;
mod utils;

#[cfg(test)]
mod test {
    use itertools::Itertools;
    use std::str::FromStr;
    use strum::IntoEnumIterator;
    use test_case::test_case;

    use crate::domains::{
        redmine, redmine_api, RedmineClient, status::AgendaStatus,
    };

    #[test_case("new" => AgendaStatus::New; "newから(insensitive)")]
    #[test_case("New" => AgendaStatus::New; "Newから")]
    #[test_case("inprogress" => AgendaStatus::InProgress; "inprogressから(insensitive)")]
    #[test_case("InProgress" => AgendaStatus::InProgress; "inprogressから")]
    #[test_case("approved" => AgendaStatus::Approved; "Approvedから(insensitive)")]
    #[test_case("Approved" => AgendaStatus::Approved; "Approvedから")]
    #[test_case("declined" => AgendaStatus::Declined; "Declinedから(insensitive)")]
    #[test_case("Declined" => AgendaStatus::Declined; "Declinedから")]
    fn agenda_status_from_str(str: &str) -> AgendaStatus {
        AgendaStatus::from_str(str).unwrap()
    }

    #[test_case(AgendaStatus::New => "🆕")]
    #[test_case(AgendaStatus::InProgress => "▶")]
    #[test_case(AgendaStatus::Approved => "⭕")]
    #[test_case(AgendaStatus::Declined => "❌")]
    fn agenda_status_to_emoji(status: AgendaStatus) -> String {
        status.emoji()
    }

    #[test_case(AgendaStatus::New => "新規")]
    #[test_case(AgendaStatus::InProgress => "進行中")]
    #[test_case(AgendaStatus::Approved => "承認")]
    #[test_case(AgendaStatus::Declined => "却下")]
    fn agenda_status_to_ja(status: AgendaStatus) -> String {
        status.ja()
    }

    #[test_case("new" => Some(AgendaStatus::New))]
    #[test_case("inp" => Some(AgendaStatus::InProgress))]
    #[test_case("app" => Some(AgendaStatus::Approved))]
    #[test_case("dec" => Some(AgendaStatus::Declined))]
    fn agenda_status_from_alias_str(str: &str) -> Option<AgendaStatus> {
        AgendaStatus::from_alias(str)
    }

    #[test_case(AgendaStatus::New => 1)]
    #[test_case(AgendaStatus::InProgress => 2)]
    #[test_case(AgendaStatus::Approved => 17)]
    #[test_case(AgendaStatus::Declined => 6)]
    fn agenda_status_id(status: AgendaStatus) -> u16 {
        status.id()
    }

    #[test]
    fn agenda_statuses_can_be_done() {
        assert_eq!(
            AgendaStatus::done_statuses(),
            vec!(AgendaStatus::Approved, AgendaStatus::Declined)
        );
    }

    #[test]
    fn test_agenda_status_contents() {
        assert_eq!(AgendaStatus::iter().count(), 4);
        assert_eq!(
            AgendaStatus::iter().collect_vec(),
            vec!(
                AgendaStatus::New,
                AgendaStatus::InProgress,
                AgendaStatus::Approved,
                AgendaStatus::Declined
            )
        );
    }

    #[tokio::test]
    async fn test_fetch_redmine_issue() {
        let mut client = RedmineClient::default();
        client
            .expect_fetch_issue()
            .returning(|_| Ok(redmine::RedmineIssueResult::default().issue));
        let redmine_api = redmine_api::RedmineApi::new(client);

        assert_eq!(
            redmine_api.fetch_issue(u16::default()).await.unwrap(),
            redmine::RedmineIssue::default()
        );
    }
}

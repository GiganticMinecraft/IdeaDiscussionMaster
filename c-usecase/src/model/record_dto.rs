use c_domain::{status::RecordStatus, Record};
use crate_shared::REDMINE_URL;

use anyhow::anyhow;
use chrono::NaiveDate;
use derive_new::new;
use regex::Regex;

#[derive(new, Debug, Clone, Default, PartialEq, Eq)]
pub struct RecordDto {
    pub id: u16,
    pub title: String,
    pub status: RecordStatus,
    pub relations: Vec<u16>,
    pub start_date: Option<NaiveDate>,
    pub due_date: Option<NaiveDate>,
}

impl RecordDto {
    pub fn formatted_id(&self) -> String {
        format!("#{}", self.id)
    }

    pub fn url(&self) -> String {
        format!("{}/issues/{}", REDMINE_URL, self.id)
    }
}

impl RecordDto {
    pub fn discussion_title(&self) -> String {
        Self::title_regex()
            .find(&self.title)
            .map(|m| m.as_str().to_string())
            .unwrap_or_else(|| "アイデア会議".to_string())
    }

    pub fn discussion_number(&self) -> anyhow::Result<u16> {
        let cap = Self::title_regex()
            .captures(&self.title)
            .ok_or_else(|| anyhow!("No matches in record title"))?;

        cap[1]
            .parse::<u16>()
            .map_err(|_| anyhow!("Error while parsing record num to u16"))
    }

    fn title_regex() -> Regex {
        Regex::new(r"第([1-9][0-9]*)回アイデア会議").unwrap()
    }
}

impl From<Record> for RecordDto {
    fn from(record: Record) -> Self {
        let relations: Vec<u16> = record.relations.into_iter().map(|id| id.into()).collect();

        Self::new(
            record.id.into(),
            record.title,
            record.status,
            relations,
            record.start_date,
            record.due_date,
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test]
    fn succeeded_in_create_title_regex() {
        RecordDto::title_regex();
    }

    #[test_case("", "アイデア会議"; "empty string should be default")]
    #[test_case("これはテストです", "アイデア会議"; "no match string should be default")]
    #[test_case("第0回アイデア会議", "アイデア会議"; "no match string with 0th should be default")]
    #[test_case("第1回アイデア会議", "第1回アイデア会議"; "1 digit number")]
    #[test_case("第10回アイデア会議", "第10回アイデア会議"; "2 digits number")]
    #[test_case("第100回アイデア会議", "第100回アイデア会議"; "3 digits number")]
    #[test_case("第65432回アイデア会議", "第65432回アイデア会議"; "5 digits number")]
    fn match_title_regex(given: &str, expected: &str) {
        let record = RecordDto {
            title: given.to_string(),
            ..RecordDto::default()
        };

        assert_eq!(record.discussion_title(), expected.to_string());
    }

    #[test_case("第1回アイデア会議" => 1; "1 digit number")]
    #[test_case("第10回アイデア会議" => 10; "2 digits number")]
    #[test_case("第100回アイデア会議" => 100; "3 digits number")]
    #[test_case("第65432回アイデア会議" => 65432; "5 digits number")]
    fn succeeded_in_match_title_number(given: &str) -> u16 {
        let record = RecordDto {
            title: given.to_string(),
            ..RecordDto::default()
        };

        record.discussion_number().unwrap()
    }

    #[test_case(""; "empty string")]
    #[test_case("これはテストです"; "no match string")]
    #[test_case("第0回アイデア会議"; "no match string with 0th")]
    #[test_case("第987654回アイデア会議"; "no match string with overflowing number")]
    fn failed_in_match_title_number(given: &str) {
        let record = RecordDto {
            title: given.to_string(),
            ..RecordDto::default()
        };

        assert!(record.discussion_number().is_err());
    }

    #[test]
    fn success_into() {
        let record = Record::default();
        let dto: RecordDto = record.clone().into();
        let relations: Vec<u16> = record
            .relations
            .clone()
            .into_iter()
            .map(|id| id.into())
            .collect();
        let expected_dto = RecordDto {
            id: record.id.into(),
            title: record.title,
            status: record.status,
            relations,
            start_date: record.start_date,
            due_date: record.due_date,
        };

        assert_eq!(dto, expected_dto);
    }
}

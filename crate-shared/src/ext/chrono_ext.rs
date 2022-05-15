use chrono::{Datelike, NaiveDate, NaiveDateTime, Weekday};

pub trait ChronoExt {
    fn weekday_ja(&self) -> String;
}

impl ChronoExt for NaiveDate {
    fn weekday_ja(&self) -> String {
        match self.weekday() {
            Weekday::Mon => "月",
            Weekday::Tue => "火",
            Weekday::Wed => "水",
            Weekday::Thu => "木",
            Weekday::Fri => "金",
            Weekday::Sat => "土",
            Weekday::Sun => "日",
        }
        .to_string()
    }
}

impl ChronoExt for NaiveDateTime {
    fn weekday_ja(&self) -> String {
        self.date().weekday_ja()
    }
}

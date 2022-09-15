use std::fmt::Display;

pub trait UseFormattedId {
    fn as_formatted_id(&self) -> String
    where
        Self: Display,
    {
        format!("#{}", self)
    }
}

impl UseFormattedId for u16 {}

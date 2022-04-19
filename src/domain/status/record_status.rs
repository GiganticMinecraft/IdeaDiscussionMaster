use strum::{Display, EnumProperty, EnumString};

#[derive(Debug, Display, EnumProperty, EnumString, PartialEq)]
pub enum RecordStatus {
    #[strum(props(id = "1"))]
    New,
    #[strum(props(id = "5"))]
    Done,
}

impl RecordStatus {
    pub fn id(&self) -> u16 {
        self.get_str("id")
            .and_then(|str| str.parse::<u16>().ok())
            .unwrap_or(1)
    }
}

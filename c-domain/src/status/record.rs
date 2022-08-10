use strum::{EnumIter, EnumProperty};

#[derive(Debug, PartialEq, Eq, Hash, Clone, EnumProperty, EnumIter)]
pub enum RecordStatus {
    #[strum(props(id = "1"))]
    New,
    #[strum(props(id = "5"))]
    Closed,
}

impl RecordStatus {
    pub fn is_new(&self) -> bool {
        *self == Self::New
    }

    pub fn is_closed(&self) -> bool {
        *self == Self::Closed
    }
}

impl Default for RecordStatus {
    fn default() -> Self {
        Self::New
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use strum::IntoEnumIterator;
    use test_case::test_case;

    #[test_case(RecordStatus::New => true; "new")]
    #[test_case(RecordStatus::Closed => false; "closed")]
    fn status_is_new(status: RecordStatus) -> bool {
        status.is_new()
    }

    #[test_case(RecordStatus::New => false; "new")]
    #[test_case(RecordStatus::Closed => true; "closed")]
    fn status_is_closed(status: RecordStatus) -> bool {
        status.is_closed()
    }

    #[test]
    fn default_status() {
        assert_eq!(RecordStatus::default(), RecordStatus::New);
    }

    #[test_case("id"; "id")]
    fn all_props_is_set(props: &str) {
        assert!(RecordStatus::iter()
            .map(|s| s.get_str(props))
            .all(|opt| opt.is_some()))
    }
}

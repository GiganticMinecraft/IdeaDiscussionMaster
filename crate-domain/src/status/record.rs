use super::StatusExt;
use strum::{EnumIter, EnumProperty};

#[derive(Debug, PartialEq, Clone, EnumProperty, EnumIter)]
pub enum RecordStatus {
    #[strum(props(id = "1"))]
    New,
    #[strum(props(id = "5"))]
    Closed,
}

impl Default for RecordStatus {
    fn default() -> Self {
        Self::New
    }
}

impl StatusExt for RecordStatus {
    fn is_new(&self) -> bool {
        *self == Self::New
    }

    fn is_closed(&self) -> bool {
        *self == Self::Closed
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case(RecordStatus::New => true; "new is true")]
    #[test_case(RecordStatus::Closed => false; "closed is false")]
    fn whether_is_new(status: RecordStatus) -> bool {
        status.is_new()
    }

    #[test_case(RecordStatus::New => false; "new is false")]
    #[test_case(RecordStatus::Closed => true; "closed is true")]
    fn whether_is_closed(status: RecordStatus) -> bool {
        status.is_closed()
    }

    #[test_case(RecordStatus::New => 1; "new")]
    #[test_case(RecordStatus::Closed => 5; "closed")]
    fn id(status: RecordStatus) -> u16 {
        status.id()
    }

    #[test_case(1 => Some(RecordStatus::New); "new")]
    #[test_case(5 => Some(RecordStatus::Closed); "closed")]
    #[test_case(50 => None; "undefined number should return None")]
    fn from_id(num: u16) -> Option<RecordStatus> {
        RecordStatus::from_id(num)
    }

    #[test]
    fn all() {
        assert_eq!(
            RecordStatus::all(),
            vec![RecordStatus::New, RecordStatus::Closed,]
        );
    }
}

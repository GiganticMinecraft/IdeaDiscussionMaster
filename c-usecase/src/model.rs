mod agenda_dto;
pub use agenda_dto::AgendaDto;

mod record_dto;
pub use record_dto::RecordDto;

mod create_record_param;
pub use create_record_param::CreateRecordParam;

pub trait DtoExt {
    fn url(&self) -> String;
}

mod agenda_dto;
pub use agenda_dto::AgendaDto;

mod record_dto;
pub use record_dto::RecordDto;

mod record_param;
pub use record_param::RecordParam;

pub trait DtoExt {
    fn url(&self) -> String;
}

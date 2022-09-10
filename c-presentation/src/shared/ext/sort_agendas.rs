use c_domain::status::AgendaStatus;
use c_usecase::model::AgendaDto;

use itertools::Itertools;

pub trait SortAgendasExt {
    fn sort_by_status(&self) -> Vec<(AgendaStatus, Vec<AgendaDto>)>;
}

impl SortAgendasExt for Vec<AgendaDto> {
    fn sort_by_status(&self) -> Vec<(AgendaStatus, Vec<AgendaDto>)> {
        let agendas = self
            .into_iter()
            .sorted_by_cached_key(|agenda| agenda.status)
            .collect_vec();

        agendas
            .into_iter()
            .group_by(|agenda| agenda.status)
            .into_iter()
            .map(|(status, group)| (status, group.cloned().collect()))
            .collect()
    }
}

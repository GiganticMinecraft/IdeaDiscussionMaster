use c_domain::redmine::model::status::AgendaStatus;
use c_usecase::model::AgendaDto;

use itertools::Itertools;

pub trait SortAgendasExt {
    fn sort_and_grouping_by_status(&self) -> Vec<(AgendaStatus, Vec<AgendaDto>)>;
    fn sort_by_id(&self) -> Vec<AgendaDto>;
}

impl SortAgendasExt for Vec<AgendaDto> {
    fn sort_and_grouping_by_status(&self) -> Vec<(AgendaStatus, Vec<AgendaDto>)> {
        // ソート
        let agendas = self
            .iter()
            .sorted_by_cached_key(|agenda| agenda.status)
            .collect_vec();

        // グループ化
        agendas
            .into_iter()
            .group_by(|agenda| agenda.status)
            .into_iter()
            .map(|(status, group)| (status, group.cloned().collect()))
            .collect()
    }

    fn sort_by_id(&self) -> Vec<AgendaDto> {
        self.iter()
            .sorted_by_key(|agenda| agenda.id)
            .cloned()
            .collect_vec()
    }
}

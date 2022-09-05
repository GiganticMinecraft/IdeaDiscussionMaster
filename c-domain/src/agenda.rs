use super::{id::AgendaId, status::AgendaStatus};

use derive_new::new;

#[derive(new, PartialEq, Eq, Debug, Default, Clone)]
pub struct Agenda {
    pub id: AgendaId,
    pub title: String,
    pub description: String,
    pub status: AgendaStatus,
}

impl Agenda {
    pub fn in_progress(self) -> anyhow::Result<Self> {
        anyhow::ensure!(
            self.status == AgendaStatus::New,
            "ステータスが「新規」である議題のみ「進行中」に変更できます"
        );

        Ok(Self {
            status: AgendaStatus::InProgress,
            ..self
        })
    }

    pub fn approve(self) -> anyhow::Result<Self> {
        anyhow::ensure!(
            vec![AgendaStatus::New, AgendaStatus::InProgress].contains(&self.status),
            "ステータスが「新規」または「進行中」である議題のみ「承認」に変更できます"
        );

        Ok(Self {
            status: AgendaStatus::Approved,
            ..self
        })
    }

    pub fn decline(self) -> anyhow::Result<Self> {
        anyhow::ensure!(
            vec![AgendaStatus::New, AgendaStatus::InProgress].contains(&self.status),
            "ステータスが「新規」または「進行中」である議題のみ「却下」に変更できます"
        );

        Ok(Self {
            status: AgendaStatus::Declined,
            ..self
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use test_case::test_case;

    #[test_case(Agenda::in_progress => AgendaStatus::InProgress; "in_progress")]
    #[test_case(Agenda::approve => AgendaStatus::Approved; "approve")]
    #[test_case(Agenda::decline => AgendaStatus::Declined; "decline")]
    fn change_status_successfully(f: fn(Agenda) -> anyhow::Result<Agenda>) -> AgendaStatus {
        let agenda = Agenda::default();
        let agenda = f(agenda);

        agenda.unwrap().status
    }

    #[test_case(Agenda::in_progress, AgendaStatus::InProgress; "in_progress when InProgress")]
    #[test_case(Agenda::in_progress, AgendaStatus::Approved; "in_progress when Approved")]
    #[test_case(Agenda::in_progress, AgendaStatus::Declined; "in_progress when Declined")]
    #[test_case(Agenda::approve, AgendaStatus::Approved; "approve when Approved")]
    #[test_case(Agenda::approve, AgendaStatus::Declined; "approve when Declined")]
    #[test_case(Agenda::decline, AgendaStatus::Declined; "decline when Declined")]
    #[test_case(Agenda::decline, AgendaStatus::Approved; "decline when Approved")]
    fn change_status_unsuccessfully(
        f: fn(Agenda) -> anyhow::Result<Agenda>,
        status_will_be_changed: AgendaStatus,
    ) {
        let agenda = Agenda {
            status: status_will_be_changed,
            ..Agenda::default()
        };
        let agenda = f(agenda);

        assert!(agenda.is_err())
    }
}

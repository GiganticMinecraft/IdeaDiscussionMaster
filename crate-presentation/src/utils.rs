pub mod discord_embeds {
    use crate_domain::id::IssueId;
    use crate_shared::{CreateEmbedExt, REDMINE_URL};
    use crate_usecase::model::AgendaDto;

    use regex::Regex;
    use serenity::builder;

    pub fn next_agenda_embed<'a>(
        embed: &'a mut builder::CreateEmbed,
        record_id: &IssueId,
        next_agenda: &AgendaDto,
    ) -> &'a mut builder::CreateEmbed {
        let reg = Regex::new(r"^\[.*\]\s").unwrap();
        let subject = reg.replace(&next_agenda.title, "");

        embed
            .custom_default(record_id)
            .simple_color()
            .title(format!("次の議題は#{}です", next_agenda.id.0))
            .field(
                "議題チケット",
                format!("{}/next_agendas/{}", REDMINE_URL, next_agenda.id.0),
                false,
            )
            .field("タイトル", subject, false)
            .field("説明", next_agenda.description.clone(), false)
    }

    pub fn no_next_agenda<'a>(
        embed: &'a mut builder::CreateEmbed,
        record_id: &IssueId,
    ) -> &'a mut builder::CreateEmbed {
        embed
            .custom_default(record_id)
            .failure_color()
            .title("次の議題はありません")
            .description("Redmine上で提起されていた議題は全て処理されました。")
    }

    // pub fn votes_result_embed(
    //     embed: &mut builder::CreateEmbed,
    //     record_id: IssueId,
    //     current_agenda_id: IssueId,
    //     status: AgendaStatus,
    // ) -> &mut builder::CreateEmbed {
    //     match status {
    //         AgendaStatus::Approved => embed.success_color().with_record_id(record_id),
    //         AgendaStatus::Declined => embed.failure_color().with_record_id(record_id),
    //         _ => embed,
    //     }
    //     .title(format!(
    //         "採決終了: #{}は{}されました",
    //         current_agenda_id.0,
    //         status.ja()
    //     ))
    // }
}

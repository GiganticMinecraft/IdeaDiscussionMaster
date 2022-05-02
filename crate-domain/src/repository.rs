mod agenda;
pub use agenda::AgendaRepository;

mod record;
pub use record::RecordRepository;

mod gh_issue;
pub use gh_issue::GitHubIssueRepository;

pub trait RepositoryModuleExt {
    type AgendaRepo: AgendaRepository;
    type RecordRepo: RecordRepository;
    type GHIssueRepo: GitHubIssueRepository;

    fn agenda_repository(&self) -> &Self::AgendaRepo;
    fn record_repository(&self) -> &Self::RecordRepo;
    fn github_issue_repository(&self) -> &Self::GHIssueRepo;
}

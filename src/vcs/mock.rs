use crate::file_system::*;
use crate::vcs::*;

type Artifact = (usize, Path, File);
type MockHistory = History<Artifact>;
type MockRepo = Repo<Artifact>;

impl RepoBackend for MockRepo {
    fn new() -> Directory<MockRepo> {
        Directory {
            label: Label::root_label(),
            entries: NonEmpty::new(DirectoryContents::Repo(Repo(Vec::new()))),
        }
    }
}

impl VCS<Artifact> for MockRepo {
    type Repository = MockRepo;
    type RepoId = MockRepo;
    type History = MockHistory;
    type HistoryId = usize; // index into repo
    type ArtefactId = usize; // index into a path

    fn get_repo(identifier: &Self::RepoId) -> Option<MockRepo> {
        Some(identifier.clone())
    }

    fn get_history(repo: &MockRepo, identifier: &Self::HistoryId) -> Option<MockHistory> {
        repo.0.get(*identifier).cloned()
    }

    fn get_histories(repo: &MockRepo) -> Vec<MockHistory> {
        repo.0.clone()
    }

    fn get_identifier(artifact: &Artifact) -> &Self::ArtefactId {
        &artifact.0
    }

    fn to_history(history: &Self::History) -> History<Artifact> {
        history.clone()
    }
}

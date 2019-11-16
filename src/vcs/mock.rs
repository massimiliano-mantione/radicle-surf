use crate::file_system::*;
use crate::vcs::*;

type MockHistory = History<Path>;
type MockRepo = Repo<Path>;

impl RepoBackend for MockRepo {
    fn new() -> Directory<MockRepo> {
        Directory {
            label: Label::root_label(),
            entries: NonEmpty::new(DirectoryContents::Repo(Repo(Vec::new()))),
        }
    }
}

impl GetRepo<Path> for MockRepo {
    type RepoId = MockRepo;
    fn get_repo(identifier: &Self::RepoId) -> Option<MockRepo> {
        Some(identifier.clone())
    }
}

impl GetHistory<Path> for MockHistory {
    type HistoryId = usize; // index into repo
    type ArtefactId = usize; // index into a path

    fn get_history(identifier: &Self::HistoryId, repo: &MockRepo) -> Option<MockHistory> {
        repo.0.get(*identifier).cloned()
    }

    fn get_identifier(_artifact: &Path) -> &Self::ArtefactId {
        unimplemented!()
    }
}

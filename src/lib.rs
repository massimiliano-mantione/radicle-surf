//! Welcome to `radicle-surf`!
//!
//! `radicle-surf` is a system to describe a file-system in a VCS world.
//! We have the concept of files and directories, but these objects can change over time while people iterate on them.
//! Thus, it is a file-system within history and we, the user, are viewing the file-system at a particular snapshot.
//! Alongside this, we will wish to take two snapshots and view their differences.
//!
//! Let's start surfing (and apologies for the `unwrap`s):
//!
//! ```
//! use radicle_surf::vcs::git::{GitBrowser, GitRepository, Sha1};
//! use radicle_surf::file_system::{Label, Path, SystemType};
//! use pretty_assertions::assert_eq;
//!
//! // We're going to point to this repo.
//! let repo = GitRepository::new(".").unwrap();
//!
//! // Here we initialise a new Broswer for a the git repo.
//! let mut browser = GitBrowser::new(&repo).unwrap();
//!
//! // Set the history to a particular commit
//! browser.commit(Sha1::new("840e75edc46c84b6392b3f38e1d830547fac89a4"))
//!        .expect("Failed to set commit");
//!
//! // Get the snapshot of the directory for our current
//! // HEAD of history.
//! let directory = browser.get_directory().unwrap();
//!
//! // Let's get a Path to this file
//! let this_file = Path::from_labels("src".into(), &["lib.rs".into()]);
//!
//! // And assert that we can find it!
//! assert!(directory.find_file(&this_file).is_some());
//!
//! let mut root_contents = directory.list_directory();
//! root_contents.sort();
//!
//! assert_eq!(root_contents, vec![
//!     SystemType::directory(".buildkite".into()),
//!     SystemType::directory(".docker".into()),
//!     SystemType::file(".gitignore".into()),
//!     SystemType::file(".gitmodules".into()),
//!     SystemType::file("Cargo.toml".into()),
//!     SystemType::file("README.md".into()),
//!     SystemType::directory("docs".into()),
//!     SystemType::directory("examples".into()),
//!     SystemType::directory("src".into()),
//! ]);
//!
//! let src = directory.find_directory(&Path::new("src".into())).unwrap();
//! let mut src_contents = src.list_directory();
//! src_contents.sort();
//!
//! assert_eq!(src_contents, vec![
//!     SystemType::directory("diff".into()),
//!     SystemType::directory("file_system".into()),
//!     SystemType::file("lib.rs".into()),
//!     SystemType::directory("vcs".into()),
//! ]);
//! ```
pub mod diff;
pub mod file_system;
pub mod vcs;

pub use crate::vcs::git;

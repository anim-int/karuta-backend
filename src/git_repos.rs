use std::path::{Path, PathBuf};

use git2::{Direction, Remote};

const MAX_CLONE_RETRIES: u32 = 10;

/// Container struct handling content stored on a git forge
///
/// This currently supports :
/// - GitHub
/// - GitLab
/// - Sourcehut
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GitSource {
    GitHub { user: String, repo: String, default_branch: String },
    GitLab { user: String, repo: String, default_branch: String },
}

impl GitSource {
    /// Creates a new GitSource from a given url
    pub fn parse_url(url: &str) -> Option<GitSource> {
        println!("Connecting to {url}");
        let mut git_remote = Remote::create_detached(url).ok()?;
        git_remote.connect(Direction::Fetch).ok()?;
        let default_branch = git_remote.default_branch().ok()?.as_str()?.into();
        println!("Got branch {default_branch}");
        if url.starts_with("https://github.com/") {
            let parts: Vec<&str> = url.split('/').collect();
            let user = parts[3].to_string();
            let repo = parts[4].trim_end_matches(".git").to_string();
            Some(GitSource::GitHub {
                user,
                repo,
                default_branch
            })
        } else if url.starts_with("https://gitlab.com/") {
            let parts: Vec<&str> = url.split('/').collect();
            let user = parts[3].to_string();
            let repo = parts[4].trim_end_matches(".git").to_string();
            Some(GitSource::GitLab {
                user,
                repo,
                default_branch
            })
        } else {
            None
        }
    }

    /// Extracts submodules from a local git repo
    pub fn from_gitmodules<P>(directory: P) -> Vec<Self>
    where
        P: AsRef<Path>,
    {
        let gitmodules = std::fs::read_to_string(directory.as_ref().join(".gitmodules")).unwrap();
        gitmodules
            .lines()
            .filter(|line| line.starts_with("\turl"))
            .map(|line| Self::parse_url(line.trim_start_matches("\turl = ")).unwrap())
            .collect()
    }

    /// Returns the url to a file given its path in the repo
    ///
    /// This uses the raw file links from the forges
    pub fn get_file_url<S>(&self, path: S) -> String
    where
        S: ToString,
    {
        match self {
            GitSource::GitLab { user, repo, default_branch } => {
                format!(
                    "https://gitlab.com/{}/{}/raw/{}/{}",
                    user,
                    repo,
                    default_branch,
                    path.to_string()
                )
            }
            GitSource::GitHub { user, repo, default_branch } => format!(
                "https://raw.githubusercontent.com/{}/{}/{}/{}",
                user,
                repo,
                default_branch,
                path.to_string()
            ),
        }
        .replace(" ", "%20")
    }

    /// Clones the repo locally in an optionally given directory, defaults to current working directory
    ///
    /// The repo is cloned to `directory/[user]_[repo]`
    pub fn clone_to_local<P>(&self, directory: Option<P>)
    where
        P: AsRef<Path>,
    {
        let directory = match directory {
            Some(dir) => dir.as_ref().to_path_buf(),
            None => std::env::current_dir().unwrap().into(),
        };
        std::fs::create_dir_all(&directory).expect("Failed to create directory");
        match self {
            GitSource::GitHub { user, repo, .. } => {
                clone_repo(
                    format!("https://github.com/{}/{}.git", user, repo),
                    directory.join(format!("{}_{}", user, repo))
                )
            }
            GitSource::GitLab { user, repo, .. } => {
                clone_repo(
                    format!("https://gitlab.com/{}/{}.git", user, repo),
                    directory.join(format!("{}_{}", user, repo))
                )
            }
        }
    }
}

fn clone_repo(source: String, target: PathBuf) {
    if target.exists() {
        // Directory already exists, skip clone
        return;
    }
    std::process::Command::new("git")
        .arg("clone")
        .arg(source)
        .arg(&target)
        .output()
        .expect("Failed to clone repository");

    let mut retries = 0;
    while !target.join(".gitmodules").exists() && retries < MAX_CLONE_RETRIES {
        println!("Waiting for clone to finish...");
        std::thread::sleep(std::time::Duration::from_secs(1)); // Wait for the clone to finish
        retries += 1;
    }
    if retries == MAX_CLONE_RETRIES {
        panic!("Failed to clone repository after {} retries", MAX_CLONE_RETRIES);
    }
}

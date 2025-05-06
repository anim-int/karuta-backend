use std::path::Path;

/// Container struct handling content stored on a git forge
/// 
/// This currently supports :
/// - GitHub
/// - GitLab
/// - Sourcehut
#[derive(Debug, Clone)]
pub enum GitSource {
    GitHub { user: String, repo: String },
    GitLab { user: String, repo: String },
    Sourcehut { user: String, repo: String },
}

impl GitSource {
    /// Creates a new GitSource from a given url
    pub fn parse_url(url: &str) -> Option<GitSource> {
        if url.starts_with("https://github.com/") {
            let parts: Vec<&str> = url.split('/').collect();
            Some(GitSource::GitHub {
                user: parts[3].to_string(),
                repo: parts[4].trim_end_matches(".git").to_string(),
            })
        } else if url.starts_with("https://gitlab.com/") {
            let parts: Vec<&str> = url.split('/').collect();
            Some(GitSource::GitLab {
                user: parts[3].to_string(),
                repo: parts[4].trim_end_matches(".git").to_string(),
            })
        } else if url.starts_with("https://git.sr.ht/") {
            let parts: Vec<&str> = url.split('/').collect();
            Some(GitSource::Sourcehut {
                user: parts[3].to_string(),
                repo: parts[4].to_string(),
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
            GitSource::Sourcehut { user, repo } => {
                format!(
                    "https://git.sr.ht/{}/{}/blob/main/{}",
                    user,
                    repo,
                    path.to_string()
                )
            }
            GitSource::GitLab { user, repo } => {
                format!(
                    "https://gitlab.com/{}/{}/raw/main/{}",
                    user,
                    repo,
                    path.to_string()
                )
            }
            GitSource::GitHub { user, repo } => format!(
                "https://raw.githubusercontent.com/{}/{}/refs/heads/main/{}",
                user,
                repo,
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
            GitSource::GitHub { user, repo } => {
                std::process::Command::new("git")
                    .arg("clone")
                    .arg(format!("https://github.com/{}/{}.git", user, repo))
                    .arg(format!("{}/{}_{}", directory.to_str().unwrap(), user, repo))
                    .output()
                    .expect("Failed to clone GitHub repository");
            }
            GitSource::GitLab { user, repo } => {
                std::process::Command::new("git")
                    .arg("clone")
                    .arg(format!("https://gitlab.com/{}/{}.git", user, repo))
                    .arg(format!("{}/{}_{}", directory.to_str().unwrap(), user, repo))
                    .output()
                    .expect("Failed to clone GitLab repository");
            }
            GitSource::Sourcehut { user, repo } => {
                std::process::Command::new("git")
                    .arg("clone")
                    .arg(format!("https://git.sr.ht/{}/{}", user, repo))
                    .arg(format!("{}/{}_{}", directory.to_str().unwrap(), user, repo))
                    .output()
                    .expect("Failed to clone SourceHut repository");
            }
        }
    }
}

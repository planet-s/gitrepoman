use std::io;
use std::path::Path;
use std::process::{Command, Stdio};
use rayon::prelude::*;

#[derive(Debug, Deserialize, Eq, Hash)]
pub struct Repo {
    pub name: String,
    pub html_url: String,
    pub ssh_url: String,
    pub namespace: String,
    pub branch: String,
}

impl Repo {
    pub fn get_url(&self) -> &str {
        &self.html_url
    }

    pub fn get_ssh_url(&self) -> &str {
        &self.ssh_url
    }
}

impl PartialEq for Repo {
    fn eq(&self, other: &Repo) -> bool {
        &self.name == &other.name
    }
}

#[derive(Debug, Fail)]
pub enum GitError {
    #[fail(display = "unable to spawn command: {}", why)]
    CommandSpawn { why: io::Error },
    #[fail(display = "returned an exit status of {}", status)]
    Failed { status: i32 }
}

fn git_cmd(args: &[&str], name: &str) -> Result<String, (String, GitError)> {
    match Command::new("git").args(args).stdout(Stdio::null()).status() {
        Ok(status) => if status.success() {
            Ok(name.to_owned())
        } else {
            Err((name.to_owned(), GitError::Failed { status: status.code().unwrap_or(1) }))
        },
        Err(why) => Err((name.to_owned(), GitError::CommandSpawn { why }))
    }
}

fn parallel_process<M, E, N>(repos: Vec<Repo>, flags: u8, namespace: &str, message: M, exists: E, new: N)
where M: Sync + Send + Fn(&Repo),
      E: Sync + Send + Fn(&Repo, &str) -> Result<String, (String, GitError)>,
      N: Sync + Send + Fn(&Repo, &str) -> Result<String, (String, GitError)>,
{
    let results = repos
        .par_iter()
        .filter(|repo| namespace == "" || repo.namespace == namespace)
        .inspect(|repo| message(repo))
        .map(|repo| {
            let url = if flags & 0b01 != 0 { repo.get_ssh_url() } else { repo.get_url() };
            if Path::new(&repo.name).exists() {
                exists(repo, url)
            } else {
                new(repo, url)
            }
        })
        .collect::<Vec<Result<String, (String, GitError)>>>();

    for result in results {
        match result {
            Ok(repo) => println!("successfully cloned {}", repo),
            Err((repo, why)) => println!("failed to clone {}: {}", repo, why)
        }
    }
}

pub trait GitAction {
    fn get_repos(&self) -> Vec<Repo>;

    fn list(&self, namespace: &str) {
        let repos = self.get_repos();

        let repos = repos.iter()
            .filter(|repo| namespace == "" || repo.namespace == namespace);

        for repo in repos {
            println!("{}: {}", repo.name, repo.ssh_url);
        }
    }

    fn clone(&self, flags: u8, namespace: &str) {
        parallel_process(
            self.get_repos(),
            flags,
            namespace,
            // Message to print before processing repo
            |repo| println!("cloning {} from {}", repo.name, repo.get_url()),
            // If repo already exists, do this
            |repo, _url| Ok(repo.name.clone()),
            // Else, do this
            |repo, url| git_cmd(&["clone", "--recursive", url, &repo.name], &repo.name)
        )
    }

    fn pull(&self, flags: u8, namespace: &str) {
        parallel_process(
            self.get_repos(),
            flags,
            namespace,
            |repo| println!("pulling {} from {}", repo.name, repo.get_url()),
            |repo, _url| {
                git_cmd(&["-C", &repo.name, "pull"], &repo.name)
                    .and_then(|_| git_cmd(&["-C", &repo.name, "submodule", "sync", "--recursive"], &repo.name))
                    .and_then(|_| git_cmd(&["-C", &repo.name, "submodule", "update", "--init"], &repo.name))
            },
            |repo, url| git_cmd(&["clone", "--recursive", url, &repo.name], &repo.name)
        )
    }

    fn checkout(&self, flags: u8, namespace: &str) {
        parallel_process(
            self.get_repos(),
            flags,
            namespace,
            |repo| println!("checking out {} from {}", repo.name, repo.get_url()),
            |repo, _url| {
                git_cmd(&["-C", &repo.name, "fetch", "origin"], &repo.name)
                    .and_then(|_| git_cmd(&["-C", &repo.name, "checkout", "-B", &repo.branch, &format!("origin/{}", repo.branch)], &repo.name))
                    .and_then(|_| git_cmd(&["-C", &repo.name, "submodule", "sync", "--recursive"], &repo.name))
                    .and_then(|_| git_cmd(&["-C", &repo.name, "submodule", "update", "--init", "--recursive"], &repo.name))
            },
            |repo, url| git_cmd(&["clone", "--recursive", url, &repo.name], &repo.name)
        )
    }

    fn mirror_pull(&self, flags: u8, namespace: &str) {
        parallel_process(
            self.get_repos(),
            flags,
            namespace,
            |repo| println!("pulling mirror {} from {}", repo.name, repo.get_url()),
            |repo, _url| git_cmd(&["-C", &repo.name, "remote", "update", "--prune"], &repo.name),
            |repo, url| git_cmd(&["clone", "--mirror", url, &repo.name], &repo.name)
        )
    }

    fn mirror_push(&self, flags: u8, namespace: &str) {
        parallel_process(
            self.get_repos(),
            flags,
            namespace,
            |repo| println!("pushing mirror {} from {}", repo.name, repo.get_url()),
            |repo, url| git_cmd(&["-C", &repo.name, "push", "--mirror", url], &repo.name),
            |repo, url| git_cmd(&["-C", &repo.name, "push", "--mirror", url], &repo.name)
        )
    }
}

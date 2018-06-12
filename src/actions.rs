use std::io;
use std::path::Path;
use std::process::Command;
use rayon::prelude::*;

#[derive(Debug, Deserialize, Eq, Hash)]
pub struct Repo {
    pub name: String,
    pub ssh_url: String
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
    match Command::new("git").args(args).status() {
        Ok(status) => if status.success() {
            Ok(name.to_owned())
        } else {
            Err((name.to_owned(), GitError::Failed { status: status.code().unwrap_or(1) }))
        },
        Err(why) => Err((name.to_owned(), GitError::CommandSpawn { why }))
    }
}

pub trait GitAction {
    fn get_repos(&self) -> Vec<Repo>;

    fn list(&self) {
        for repo in self.get_repos() {
            println!("{}: {}", repo.name, repo.ssh_url);
        }
    }

    fn clone(&self) {
        let results = self.get_repos()
            .par_iter()
            .inspect(|repo| println!("cloning {}", repo.name))
            .map(|repo| if !Path::new(&repo.name).exists() {
                git_cmd(&["clone", "--recursive", &repo.ssh_url, &repo.name], &repo.name)
            } else {
                Ok(repo.name.clone())
            })
            .collect::<Vec<Result<String, (String, GitError)>>>();

        for result in results {
            match result {
                Ok(repo) => println!("successfully cloned {}", repo),
                Err((repo, why)) => println!("failed to clone {}: {}", repo, why)
            }
        }
    }

    fn pull(&self) {
        let results = self.get_repos()
            .par_iter()
            .inspect(|repo| println!("pulling {}", repo.name))
            .map(|repo| if !Path::new(&repo.name).exists() {
                git_cmd(&["clone", "--recursive", &repo.ssh_url, &repo.name], &repo.name)
            } else {
                git_cmd(&["-C", &repo.name, "pull"], &repo.name)
                    .and_then(|_| git_cmd(&["-C", &repo.name, "submodule", "sync", "--recursive"], &repo.name))
                    .and_then(|_| git_cmd(&["-C", &repo.name, "submodule", "update", "--init"], &repo.name))
            })
            .collect::<Vec<Result<String, (String, GitError)>>>();

        for result in results {
            match result {
                Ok(repo) => println!("successfully pulled {}", repo),
                Err((repo, why)) => println!("failed to pull {}: {}", repo, why)
            }
        }
    }

    fn checkout(&self) {
        let results = self.get_repos()
            .par_iter()
            .inspect(|repo| println!("checking out {}", repo.name))
            .map(|repo| if !Path::new(&repo.name).exists() {
                git_cmd(&["clone", "--recursive", &repo.ssh_url, &repo.name], &repo.name)
            } else {
                git_cmd(&["-C", &repo.name, "fetch", "origin"], &repo.name)
                    .and_then(|_| git_cmd(&["-C", &repo.name, "submodule", "sync", "--recursive"], &repo.name))
                    .and_then(|_| git_cmd(&["-C", &repo.name, "submodule", "update", "--init", "--recursive"], &repo.name))
            })
            .collect::<Vec<Result<String, (String, GitError)>>>();

        for result in results {
            match result {
                Ok(repo) => println!("successfully pulled {}", repo),
                Err((repo, why)) => println!("failed to pull {}: {}", repo, why)
            }
        }
    }
}

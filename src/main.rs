extern crate gitlab;
extern crate github_rs;
extern crate toml;
#[macro_use] extern crate serde_derive;
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate rayon;
extern crate serde_json;

mod action;
mod config;
mod gitlab_impl;
mod github_impl;

use self::action::Action;
use self::config::*;
use self::github_impl::GitHub;
use std::process::{exit, Command, Stdio};
use std::env;
use std::io;
use std::path::Path;
use gitlab::Gitlab;
use rayon::prelude::*;

#[derive(Debug, Deserialize)]
pub struct Repo {
    name: String,
    ssh_url: String
}

#[derive(Debug, Fail)]
pub enum GitError {
    #[fail(display = "unable to spawn command: {}", why)]
    CommandSpawn { why: io::Error },
    #[fail(display = "returned an exit status of {}", status)]
    Failed { status: i32 }
}

fn git_cmd(args: &[&str], name: &str) -> Result<String, (String, GitError)> {
    match Command::new("git").args(args).stdout(Stdio::null()).stderr(Stdio::null()).status() {
        Ok(status) => if status.success() {
            Ok(name.to_owned())
        } else {
            Err((name.to_owned(), GitError::Failed { status: status.code().unwrap_or(1) }))
        },
        Err(why) => Err((name.to_owned(), GitError::CommandSpawn { why }))
    }
}

pub trait GitAccount {
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

fn main() {
    let mut cmds = env::args().skip(1);
    let config = match Config::new() {
        Ok(config) => config,
        Err(why) => {
            eprintln!("failed to get config: {}", why);
            exit(1);
        }
    };

    let (source, org) = match (cmds.next(), cmds.next()) {
        (Some(source), Some(org)) => (source, org),
        _ => {
            eprintln!("gitlab DOMAIN / github ORG_ID");
            exit(1);
        }
    };

    let action = match cmds.next() {
        Some(action) => action,
        None => {
            eprintln!("supported actions: list, clone, pull");
            exit(1);
        }
    };

    let authenticated: Box<GitAccount> = match GitService::from(source.as_str()) {
        Some(GitService::GitHub) => {
            let token = match config.github {
                Some(token) => token,
                None => {
                    eprintln!("no GitHub token provided");
                    exit(1);
                }
            };

            match GitHub::new(org, token) {
                Ok(client) => Box::new(client),
                Err(why) => {
                    eprintln!("unable to authenticate client: {}", why);
                    exit(1);
                }
            }
        },
        Some(GitService::GitLab) => {
            let token = match config.gitlab {
                Some(token) => token,
                None => {
                    eprintln!("no GitLab token provided");
                    exit(1);
                }
            };

            match Gitlab::new(org, token) {
                Ok(client) => Box::new(client),
                Err(why) => {
                    eprintln!("unable to authenticate client: {}", why);
                    exit(1);
                }
            }
        },
        None => {
            eprintln!("invalid source: {}", source);
            exit(1);
        }
    };

    match Action::from(action.as_str()) {
        Ok(Action::List) => authenticated.list(),
        Ok(Action::Clone) => authenticated.clone(),
        Ok(Action::Pull) => authenticated.pull(),
        Ok(Action::Checkout) => authenticated.checkout(),
        Err(cmd) => {
            eprintln!("{} is not a valid command", cmd);
            exit(1);
        }
    }
}

use github_rs::client::Executor;
use github_rs::client::Github as GithubApi;
use github_rs::errors::Error as GithubError;
use std::collections::HashSet;
use actions::{GitAction, Repo};

pub struct GitHub {
    client: GithubApi,
    org: String,
}

impl GitHub {
    pub fn new(org: String, token: String) -> Result<GitHub, GithubError> {
        Ok(GitHub { client: GithubApi::new(token)?, org })
    }
}

#[derive(Debug, Deserialize)]
pub struct GithubRepo {
    pub name: String,
    pub html_url: String,
    pub ssh_url: String,
    pub default_branch: String,
}

impl GitAction for GitHub {
    fn get_repos(&self) -> Vec<Repo> {
        let mut output = HashSet::new();
        for page in 0.. {
            let request = self.client.get()
                .custom_endpoint(&format!("orgs/{}/repos?page={}", self.org, page))
                .execute::<Vec<GithubRepo>>();

            if let Ok((_, _, Some(repos))) = request {
                if repos.len() > 0 {
                    for repo in repos {
                        output.insert(Repo {
                            name: repo.name,
                            html_url: repo.html_url,
                            ssh_url: repo.ssh_url,
                            namespace: "".into(),
                            branch: repo.default_branch,
                        });
                    }
                } else {
                    break
                }
            } else {
                break
            }
        }


        output.into_iter().collect()
    }
}

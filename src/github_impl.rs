use github_rs::client::Executor;
use github_rs::client::Github as GithubApi;
use github_rs::errors::Error as GithubError;
use ::{GitAccount, Repo};

pub struct GitHub {
    client: GithubApi,
    org: String,
}

impl GitHub {
    pub fn new(org: String, token: String) -> Result<GitHub, GithubError> {
        Ok(GitHub { client: GithubApi::new(token)?, org })
    }
}

impl GitAccount for GitHub {
    fn get_repos(&self) -> Vec<Repo> {
        let mut output = Vec::new();
        for page in 0.. {
            let request = self.client.get()
                .custom_endpoint(&format!("orgs/{}/repos?page={}", self.org, page))
                .execute::<Vec<Repo>>();

            if let Ok((_, _, Some(repos))) = request {
                if repos.len() > 0 {
                    for repo in repos {
                        output.push(repo);
                    }
                } else {
                    break
                }
            } else {
                break
            }
        }


        output
    }
}

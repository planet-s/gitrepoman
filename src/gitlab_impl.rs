use gitlab::Gitlab;
use actions::{GitAction, Repo};

impl GitAction for Gitlab {
    fn get_repos(&self) -> Vec<Repo> {
        let mut repos = Vec::new();

        if let Ok(projects) = self.projects() {
            for project in projects {
                repos.push(Repo {
                    name: project.name,
                    html_url: project.http_url_to_repo,
                    ssh_url: project.ssh_url_to_repo
                })
            }
        }

        repos
    }
}

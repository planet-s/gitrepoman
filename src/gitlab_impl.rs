use gitlab::Gitlab;
use ::{GitAccount, Repo};

impl GitAccount for Gitlab {
    fn get_repos(&self) -> Vec<Repo> {
        let mut repos = Vec::new();

        if let Ok(projects) = self.projects() {
            for project in projects {
                repos.push(Repo {
                    name: project.name,
                    ssh_url: project.ssh_url_to_repo
                })
            }
        }

        repos
    }
}

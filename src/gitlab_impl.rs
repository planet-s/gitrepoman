use gitlab::Gitlab;
use actions::{GitAction, Repo};

impl GitAction for Gitlab {
    fn get_repos(&self) -> Vec<Repo> {
        let mut repos = Vec::new();

        let params: &[(&str, &str)] = &[];
        if let Ok(projects) = self.projects(params) {
            for project in projects {
                repos.push(Repo {
                    name: project.name,
                    html_url: project.http_url_to_repo,
                    ssh_url: project.ssh_url_to_repo,
                    namespace: project.namespace.full_path,
                    branch: project.default_branch.unwrap_or("master".to_string())
                })
            }
        }

        repos
    }
}

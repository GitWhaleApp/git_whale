mod git_data;
use git2::Repository;
use git_data::Repo;

pub fn open_repo(path: &str) -> Repo<Repository> {
    match Repository::open(path) {
        Ok(repo) => {
            Repo{data: repo, path: String::from(path)}
        },
        Err(e) => {
            panic!("Error: {}", e);
        }
    }
}

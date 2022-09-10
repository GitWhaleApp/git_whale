mod services;
use services::git_adapter;

fn get_version() -> String {
    String::from(env!("CARGO_PKG_VERSION"))
}


fn main() {

    let git_repo = git_adapter::open_repo("/Users/rudrabhoj/Rust/git_whale");

    println!("Welcome to git_whale v{}", get_version());
    println!("Repo: {:?}", git_repo.path);
}


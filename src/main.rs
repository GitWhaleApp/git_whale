mod services;
use services::git_adapter;

fn get_version() -> String {
    String::from(env!("CARGO_PKG_VERSION"))
}


fn main() {

    let git_repo = git_adapter::open_repo("/Users/rudrabhoj/Work/GitWhale");

    println!("Welcome to GitWhale v{}", get_version());
    println!("Repo: {:?}", git_repo.path);
}


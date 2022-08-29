fn get_version() -> String {
    String::from(env!("CARGO_PKG_VERSION"))
}


fn main() {
    println!("Welcome to GitWhale v{}", get_version());
}


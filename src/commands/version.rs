pub fn print_version() {
    const NAME: &str = env!("CARGO_PKG_NAME");
    const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
    println!(
        "{} v{}
By: {}
Repoistory: {}",
        NAME, VERSION, AUTHORS, REPOSITORY
    );
}

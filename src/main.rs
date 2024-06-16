mod cli;
mod model;
mod schema;

fn main() {
    use clap::Parser;
    use cli::Args;
    Args::parse();
}

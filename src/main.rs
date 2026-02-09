mod cli;
mod db;
mod operation;
mod todo;
mod utils;

fn main() -> eyre::Result<()> {
    pretty_env_logger::init();
    color_eyre::install()?;
    cli::run()
}

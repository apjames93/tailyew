use anyhow::Result;
use clap::Parser;
use create_tailyew_app::{scaffold_frontend, Config};

fn main() -> Result<()> {
    let cfg = Config::parse();
    scaffold_frontend(cfg)
}

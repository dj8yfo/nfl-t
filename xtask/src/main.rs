use std::path::PathBuf;

use anyhow::{anyhow, Result};

use clap::Parser;

use console::style;
use duct::cmd;

#[derive(clap::Subcommand, Debug)]
enum Action {
    Check,
    RunWithAsserts,
}

/// simple program to demonstrate oracle feed pallet
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    action: Action,
}

fn switch_to_workspace_root() -> Result<()> {
    std::env::set_current_dir(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .ok_or_else(|| anyhow!("failed to find the workspace root"))?,
    )?;
    Ok(())
}

fn fmt() -> Result<()> {
    println!("{}", style("cargo fmt").blue());
    cmd!("cargo", "fmt").run()?;
    Ok(())
}

fn check() -> Result<()> {
    println!("{}", style("cargo check").blue());
    cmd!("cargo", "check", "--all-targets").run()?;
    Ok(())
}

fn clippy() -> Result<()> {
    println!("{}", style("cargo clippy").blue());
    cmd!("cargo", "clippy", "--all-targets").run()?;
    Ok(())
}

fn demo_with_asserts() -> Result<()> {
    println!("{}", style("cargo engine").blue());
    cmd!("cargo", "run", "-p", "oracle_feed_demo").run()?;
    Ok(())
}
fn main() -> Result<()> {
    let args = Args::parse();

    match args.action {
        Action::Check => {
            switch_to_workspace_root()?;
            fmt()?;
            check()?;
            clippy()?;
        }
        Action::RunWithAsserts => {
            switch_to_workspace_root()?;
            demo_with_asserts()?;
        }
    }

    Ok(())
}

// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Generates the `wiserone` manpages and shell completions from the CLI
//! definition, so they can never drift from `--help`.
//!
//! Nothing it writes is committed. `make assets` and the release workflow
//! run it, and each release archive ships the result.
//!
//! ```text
//! cargo run --example gen_assets -- <out-dir>
//! ```
//!
//! Writes `<out-dir>/man/*.1` (one page per command) and
//! `<out-dir>/completions/` for bash, zsh, fish, PowerShell and elvish.

use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use clap::CommandFactory;
use clap_complete::Shell;
use wiserone::cli::Command;

fn generate(out: &Path) -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::command().name("wiserone");

    let man = out.join("man");
    fs::create_dir_all(&man)?;
    clap_mangen::generate_to(cmd.clone(), &man)?;

    let completions = out.join("completions");
    fs::create_dir_all(&completions)?;
    for shell in [
        Shell::Bash,
        Shell::Zsh,
        Shell::Fish,
        Shell::PowerShell,
        Shell::Elvish,
    ] {
        let _script = clap_complete::generate_to(
            shell,
            &mut cmd,
            "wiserone",
            &completions,
        )?;
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let out = std::env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .ok_or("usage: gen_assets <out-dir>")?;
    generate(&out)?;
    println!("wrote manpages and completions to {}", out.display());
    Ok(())
}

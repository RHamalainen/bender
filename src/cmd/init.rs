// Copyright (c) 2021 ETH Zurich
// Michael Rogenmoser <michaero@iis.ee.ethz.ch>

//! The `init` subcommand.

use std::env::current_dir;
use std::ffi::OsStr;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command as SysCommand;

use clap::{ArgMatches, Command};

use crate::error::*;

/// Assemble the `init` subcommand.
pub fn new() -> Command {
    Command::new("init").about("Initialize a Bender package")
}

/// Execute the `init` subcommand.
pub fn run(_matches: &ArgMatches) -> Result<()> {
    // Get working directory name
    let binding = current_dir()?;
    let cwd = binding
        .as_path()
        .file_name()
        .unwrap_or(OsStr::new("new_package"))
        .to_str()
        .unwrap_or("new_package");

    // Get author from git config
    let name = String::from_utf8(
        SysCommand::new("git")
            .args(["config", "user.name"])
            .output()?
            .stdout,
    )
    .unwrap_or("".to_string());
    let name = &name
        .strip_suffix("\r\n")
        .unwrap_or(name.strip_suffix('\n').unwrap_or(&name));
    let email = String::from_utf8(
        SysCommand::new("git")
            .args(["config", "user.email"])
            .output()?
            .stdout,
    )
    .unwrap_or("".to_string());
    let email = &email
        .strip_suffix("\r\n")
        .unwrap_or(email.strip_suffix('\n').unwrap_or(&email));

    // Create Bender.yml
    if Path::new("Bender.yml").exists() {
        return Err(Error::new("Bender.yml already exists"));
    }

    let mut file = File::create("Bender.yml")?;

    writeln!(
        file,
        "\
package:
  name: {}
  authors:
    - \"{} <{}>\"

dependencies:

sources:
  # Source files grouped in levels. Files in level 0 have no dependencies on files in this
  # package. Files in level 1 only depend on files in level 0, files in level 2 on files in
  # levels 1 and 0, etc. Files within a level are ordered alphabetically.
  # Level 0",
        cwd, name, email
    )?;

    Ok(())
}

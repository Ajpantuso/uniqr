// SPDX-FileCopyrightText: 2023 Andrew Pantuso <ajpantuso@gmail.com>
//
// SPDX-License-Identifier: Apache-2.0

use clap::Parser;
use std::process;
use uniqr::options;
use uniqr::Command;

fn main() {
    let opts = options::Options::parse();

    if let Err(e) = Command::from(&opts).run() {
        eprintln!("{e}");

        process::exit(1);
    }
}

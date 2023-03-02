// SPDX-FileCopyrightText: 2023 Andrew Pantuso <ajpantuso@gmail.com>
//
// SPDX-License-Identifier: Apache-2.0

use clap::Parser;

#[derive(Parser)]
pub struct Options {
    #[arg(default_value = "-")]
    pub input: String,
    #[arg()]
    pub output: Option<String>,
    #[arg(short = 'c')]
    #[arg(conflicts_with = "all_duplicated")]
    pub count: bool,
    #[arg(short = 'u')]
    pub unique: bool,
    #[arg(short = 'd')]
    pub duplicated: bool,
    #[arg(short = 'D')]
    pub all_duplicated: bool,
}

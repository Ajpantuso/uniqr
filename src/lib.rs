// SPDX-FileCopyrightText: 2023 Andrew Pantuso <ajpantuso@gmail.com>
//
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use std::fs;
use std::io::{self, Write};

mod input;
mod line_count;
pub mod options;

pub struct Command<'a> {
    options: &'a options::Options,
}

impl<'a> Command<'a> {
    pub fn run(&self) -> Result<()> {
        let mut out: Box<dyn Write> = if self.options.output.is_some() {
            Box::new(fs::File::create(self.options.output.as_ref().unwrap())?)
        } else {
            Box::new(io::stdout().lock())
        };

        let f = input::open(&self.options.input)?;
        let counts = line_count::OrderedLineCount::try_from(f)?;

        let lines: Box<dyn Iterator<Item = String>> = if self.options.all_duplicated {
            Box::new(counts.all_dups())
        } else {
            let with_counts: Box<dyn Iterator<Item = (usize, String)>> = if self.options.duplicated
            {
                Box::new(counts.only_dups())
            } else if self.options.unique {
                Box::new(counts.only_unique())
            } else {
                Box::new(counts.deduped())
            };

            if self.options.count {
                Box::new(with_counts.map(|e| format!("{:>4} {}", e.0, e.1)))
            } else {
                Box::new(with_counts.map(|e| e.1))
            }
        };

        for line in lines {
            write!(out, "{line}")?
        }

        Ok(())
    }
}

impl<'a> From<&'a options::Options> for Command<'a> {
    fn from(options: &'a options::Options) -> Self {
        Command { options }
    }
}

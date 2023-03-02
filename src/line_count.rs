// SPDX-FileCopyrightText: 2023 Andrew Pantuso <ajpantuso@gmail.com>
//
// SPDX-License-Identifier: Apache-2.0

use crate::input::Input;
use anyhow::{self, Result};
use itertools::Itertools;
use std::io;
use std::iter;

#[derive(Debug)]
pub struct OrderedLineCount<I>
where
    I: Iterator<Item = io::Result<String>>,
{
    inner: iter::Peekable<I>,
}

impl<I> OrderedLineCount<I>
where
    I: Iterator<Item = io::Result<String>>,
{
    pub fn deduped(self) -> impl Iterator<Item = (usize, String)> {
        self
    }
    pub fn only_unique(self) -> impl Iterator<Item = (usize, String)> {
        self.filter(|e| e.0 == 1)
    }
    pub fn only_dups(self) -> impl Iterator<Item = (usize, String)> {
        self.filter(|e| e.0 > 1)
    }
    pub fn all_dups(self) -> impl Iterator<Item = String> {
        self.filter(|e| e.0 > 1)
            .flat_map(|e| iter::repeat(e.1).take(e.0))
    }
}

impl<I: Iterator<Item = io::Result<String>>> Iterator for OrderedLineCount<I> {
    type Item = (usize, String);
    fn next(&mut self) -> Option<Self::Item> {
        let inner = self.inner.by_ref();

        let line = match inner.next() {
            Some(Ok(l)) => l,
            _ => return None,
        };

        let count = 1 + inner
            .peeking_take_while(|next| match next {
                Ok(n) => line == *n,
                Err(_) => false,
            })
            .count();

        Some((count, line))
    }
}

impl TryFrom<Box<dyn Input>> for OrderedLineCount<Box<dyn Iterator<Item = io::Result<String>>>> {
    type Error = anyhow::Error;

    fn try_from(reader: Box<dyn Input>) -> Result<Self, Self::Error> {
        let inner = reader.to_lines()?.peekable();

        Ok(Self { inner })
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::input;
    use test_case::test_case;

    #[test_case("a\nb\nb\nc\n", vec![(1, String::from("a\n")), (2, String::from("b\n")), (1, String::from("c\n"))]; "happy path")]
    #[test_case("a\n\n\nc\n", vec![(1, String::from("a\n")), (2, String::from("\n")), (1, String::from("c\n"))]; "blanks")]
    #[test_case("a\nb\nc\n", vec![(1, String::from("a\n")), (1, String::from("b\n")), (1, String::from("c\n"))]; "no dups")]
    fn try_from_bufreader(input: &str, output: Vec<(usize, String)>) {
        let input: Box<dyn Input> = Box::new(input::BufferedInput::new(
            "test",
            input::StringOpener::new(input),
        ));

        if let Ok(olc) = OrderedLineCount::try_from(input) {
            assert_eq!(output, olc.collect::<Vec<(usize, String)>>())
        }
    }
}

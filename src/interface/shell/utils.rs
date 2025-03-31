use alloc::vec::Vec;
use thiserror::Error;

use super::parse::Args;

#[derive(Debug)]
pub struct ArgParser<'a, const N: usize> {
    pub positionals: [&'a str; N],
    pub named: Vec<(&'a str, Option<&'a str>)>,
}

impl<const N: usize> ArgParser<'_, N> {
    pub fn validate(&self, keys: Vec<&str>) -> Result<Self, ArgParseError> {
        todo!()
    }
}

impl<const N: usize> TryFrom<Args> for ArgParser<'_, N> {
    type Error = ArgParseError;
    fn try_from(value: Args) -> Result<Self, Self::Error> {
        todo!()
    }
}

#[derive(Debug, Error)]
pub enum ArgParseError {}

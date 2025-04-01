use core::marker::PhantomData;

use alloc::{format, string::String, vec::Vec};
use thiserror::Error;

use super::parse::Args;

#[derive(Debug)]
pub struct ArgParser<'a, const N: usize> {
    pub positionals: [String; N],
    pub named: Vec<(String, Option<String>)>,
    pin: PhantomData<&'a [u8]>,
}

impl<const N: usize> ArgParser<'_, N> {
    pub fn validate(mut self, keys: Vec<&str>) -> Result<Self, ArgParseError> {
        let mut new = keys.clone();
        for (name, _) in self.named.iter() {
            if !keys.contains(&&name[..]) {
                return Err(ArgParseError::NamedArgError(format!(
                    "wrong arg supplied, {} not known",
                    name
                )));
            } else {
                new.remove(new.iter().position(|n| n == name).expect("impossible"));
            }
        }
        for name in new {
            self.named.push((name.into(), None));
        }
        Ok(self)
    }
}

impl<const N: usize> TryFrom<Args> for ArgParser<'_, N> {
    type Error = ArgParseError;
    fn try_from(value: Args) -> Result<Self, Self::Error> {
        if value.positional.len() != N {
            return Err(ArgParseError::PositionalArgError(format!(
                "wrong number of positional args, expected {}, got {}",
                N,
                value.positional.len()
            )));
        }
        let positionals: [String; N] = match value.positional.try_into() {
            Ok(v) => v,
            Err(_) => panic!("impossible"),
        };
        let named = value
            .named
            .into_iter()
            .map(move |arg| {
                (
                    arg.name,
                    if arg.val.is_empty() {
                        None
                    } else {
                        Some(arg.val)
                    },
                )
            })
            .collect();
        Ok(Self {
            positionals,
            named,
            pin: PhantomData,
        })
    }
}

#[derive(Debug, Error)]
pub enum ArgParseError {
    #[error("wrong positional arguments supplied due to {0}")]
    PositionalArgError(String),
    #[error("wring named arguments supplied due to {0}")]
    NamedArgError(String),
}

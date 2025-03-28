use alloc::{string::String, vec::Vec};
use core::{fmt::Display, ops::Index};
use thiserror::Error;
mod layout;
mod operations;
pub use operations::{init, insert_dir, insert_file, read, remove, write};

pub type Result<T> = core::result::Result<T, FSError>;

#[derive(Error, Debug)]
pub enum FSError {
    #[error("the node {0} does not exist")]
    NodeNotFound(PathBuf),
    #[error("the node {0} already exists")]
    NodeExists(PathBuf),
    #[error("the node {0} does not implement operation {1}")]
    NotImplemented(String, String),
}

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct Path<'a> {
    elements: Vec<&'a str>,
}

impl<'a> Path<'a> {
    pub fn new(p: &'a str) -> Self {
        let s = p.split('/').collect::<Vec<&'a str>>();
        Self { elements: s }
    }

    pub fn split_last(&self) -> (Self, Self) {
        if self.elements.is_empty() {
            return (Self::new(""), Self::new(""));
        }
        let (first, last) = self.elements.split_at(self.elements.len() - 1);
        (
            Self {
                elements: first.to_vec(),
            },
            Self {
                elements: last.to_vec(),
            },
        )
    }
    pub fn get(&self, i: usize) -> Option<&'a str> {
        if i >= self.elements.len() {
            None
        } else {
            Some(self.elements[i])
        }
    }
}

impl<'a> Index<usize> for Path<'a> {
    type Output = str;
    fn index(&self, index: usize) -> &Self::Output {
        self.elements[index]
    }
}

impl<'a> Display for Path<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct PathBuf {
    elements: Vec<String>,
}

impl Display for PathBuf {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Ok(())
    }
}

impl<'a> From<Path<'a>> for PathBuf {
    fn from(value: Path) -> Self {
        let p = value
            .elements
            .into_iter()
            .map(|p| p.into())
            .collect::<Vec<String>>();
        Self { elements: p }
    }
}

impl<'a> From<&Path<'a>> for PathBuf {
    fn from(value: &Path) -> Self {
        let p = value
            .elements
            .clone()
            .into_iter()
            .map(|p| p.into())
            .collect::<Vec<String>>();
        Self { elements: p }
    }
}

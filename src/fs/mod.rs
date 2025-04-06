use alloc::{string::String, vec::Vec};
use core::{fmt::Display, ops::Index};
use thiserror::Error;
mod layout;
mod operations;
pub use operations::{get_node_info, init, insert_dir, insert_file, read, remove, write};

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
        let s = p
            .split('/')
            .filter(|s| !s.is_empty())
            .collect::<Vec<&'a str>>();
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
    pub fn try_concat(p1: &'a Path, p2: &'a Path) -> Self {
        // TODO proper relative part resolution
        // for now assuming, that p2 starts in . as ./...
        let mut p = p1.clone();
        p.extend(&p2.elements[..]);
        p
    }
    pub fn extend(&mut self, elements: &[&'a str]) {
        self.elements.extend_from_slice(elements);
    }
    pub fn canonicalize(&self) -> Self {
        // TODO
        let mut s = Self::new("");
        for element in &self.elements {
            if *element != "." && *element != ".." {
                s.extend(&[element]);
            } else if *element == ".." {
                (s, _) = s.split_last();
            }
        }
        s
    }
}

impl Index<usize> for Path<'_> {
    type Output = str;
    fn index(&self, index: usize) -> &Self::Output {
        self.elements[index]
    }
}

impl Display for Path<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.elements.is_empty() {
            write!(f, "/")?;
        }
        for e in &self.elements {
            write!(f, "/{}", e)?;
        }
        writeln!(f)?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct PathBuf {
    elements: Vec<String>,
}

impl PathBuf {
    pub fn new(p: String) -> Self {
        let s = p
            .split('/')
            .filter(|s| !s.is_empty())
            .map(|s| s.into())
            .collect::<Vec<String>>();
        Self { elements: s }
    }

    pub fn as_path(&self) -> Path {
        Path {
            elements: self.elements.iter().map(|s| s.as_str()).collect(),
        }
    }

    pub fn canonicalize(&mut self) {
        // TODO
    }
}

impl Display for PathBuf {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.elements.is_empty() {
            write!(f, "/")?;
        }
        for e in &self.elements {
            write!(f, "/{}", e)?;
        }
        Ok(())
    }
}

impl From<Path<'_>> for PathBuf {
    fn from(value: Path) -> Self {
        let p = value
            .elements
            .into_iter()
            .map(|p| p.into())
            .collect::<Vec<String>>();
        Self { elements: p }
    }
}

impl From<&Path<'_>> for PathBuf {
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

impl<'a> From<&'a PathBuf> for Path<'a> {
    fn from(value: &'a PathBuf) -> Self {
        Path {
            elements: value.elements.iter().map(|i| i.as_str()).collect(),
        }
    }
}

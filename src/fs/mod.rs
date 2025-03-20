use alloc::{
    rc::Rc,
    string::String,
    vec::{self, Vec},
};
use core::{fmt::Display, slice};
use layout::DirNode;
use thiserror::Error;

mod layout;
mod operations;

pub type Result<T> = core::result::Result<T, FSError>;

#[derive(Error, Debug)]
pub enum FSError {
    #[error("the node {0} does not exist")]
    NodeNotFound(Path),
    #[error("the node {0} already exists")]
    NodeExists(Path),
}

pub struct Fs {
    root: Rc<DirNode>,
}

impl Fs {
    fn new() -> Self {
        Self {
            root: Rc::new(DirNode::new()),
        }
    }
    fn get(&self, p: Path) -> Rc<impl FSNode> {
        self.root.clone()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Path {
    p: Vec<String>,
}

impl Display for Path {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Ok(())
    }
}

pub trait FSNode {}

pub struct FSNodeData {
    data: FSMetaData,
    node: Rc<dyn FSNode>,
}

struct FSMetaData {}

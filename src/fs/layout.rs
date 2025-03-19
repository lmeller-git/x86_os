use super::operations::{Visitable, Visitor};
use alloc::{string::String, vec::Vec};
use core::fmt::{Debug, Display};

#[derive(Clone)]
pub(in crate::fs) struct INode {
    pub node_type: NodeType,
    pub name: String,
}

impl INode {
    pub fn new(node_type: NodeType, name: String) -> Self {
        Self { node_type, name }
    }
    pub fn inner(&self) -> &NodeType {
        &self.node_type
    }
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn mut_inner(&mut self) -> &mut NodeType {
        &mut self.node_type
    }
}

impl<V> Visitable<V> for INode
where
    V: Visitor,
{
    fn accept(&self, visitor: &mut V) -> V::Output {
        visitor.visit_inode(self)
    }
    fn accept_mut(&mut self, visitor: &mut V) -> V::Output {
        visitor.visit_inode_mut(self)
    }
}

#[derive(Clone)]
pub enum NodeType {
    File(File),
    Dir(Dir),
}

impl From<File> for NodeType {
    fn from(value: File) -> Self {
        Self::File(value)
    }
}

impl From<Dir> for NodeType {
    fn from(value: Dir) -> Self {
        Self::Dir(value)
    }
}

#[derive(Clone)]
pub struct File {
    pub content: String,
}

impl File {
    fn new() -> Self {
        File {
            content: String::new(),
        }
    }
}

impl<V> Visitable<V> for File
where
    V: Visitor,
{
    fn accept(&self, visitor: &mut V) -> V::Output {
        visitor.visit_file(self)
    }
    fn accept_mut(&mut self, visitor: &mut V) -> V::Output {
        visitor.visit_file_mut(self)
    }
}

#[derive(Clone)]
pub struct Dir {
    pub children: Vec<INode>,
}

impl Dir {
    fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl<V> Visitable<V> for Dir
where
    V: Visitor,
{
    fn accept(&self, visitor: &mut V) -> V::Output {
        visitor.visit_dir(self)
    }
    fn accept_mut(&mut self, visitor: &mut V) -> V::Output {
        visitor.visit_dir_mut(self)
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Path {}

impl Display for Path {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Ok(())
    }
}

impl Debug for Path {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Ok(())
    }
}

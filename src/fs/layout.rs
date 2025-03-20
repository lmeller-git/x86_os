use alloc::{rc::Rc, vec::Vec};

use super::FSNode;

pub struct DirNode {
    pub children: Vec<Rc<dyn FSNode>>,
}

impl DirNode {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl FSNode for DirNode {}

pub struct FileNode {
    data: FileData,
}

impl FSNode for FileNode {}

pub struct FileData {}

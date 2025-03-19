use alloc::string::String;

use crate::fs::{
    layout::{NodeType, Path},
    FSError, Result,
};

use super::{Visitable, Visitor};

pub struct FileReader {
    p: Path,
}

impl FileReader {
    fn new(p: Path) -> Self {
        Self { p }
    }
}

impl Visitor for FileReader {
    type Output = Result<String>;
    fn visit_inode(&mut self, inode: &crate::fs::layout::INode) -> Self::Output {
        match &inode.node_type {
            NodeType::File(f) => f.accept(self),
            NodeType::Dir(d) => d.accept(self),
        }
    }

    fn visit_file(&mut self, file: &crate::fs::layout::File) -> Self::Output {
        todo!()
    }

    fn visit_dir(&mut self, dir: &crate::fs::layout::Dir) -> Self::Output {
        for child in &dir.children {
            match child.accept(self) {
                Err(_) => {}
                Ok(v) => return Ok(v),
            }
        }
        Err(FSError::NodeNotFound(self.p.clone()))
    }
}

pub struct FileWriter {
    p: Path,
}

impl FileWriter {
    fn new(p: Path) -> Self {
        Self { p }
    }
}

impl Visitor for FileWriter {
    type Output = Result<()>;

    fn visit_inode(&mut self, inode: &crate::fs::layout::INode) -> Self::Output {
        todo!()
    }

    fn visit_file(&mut self, file: &crate::fs::layout::File) -> Self::Output {
        todo!()
    }

    fn visit_dir(&mut self, dir: &crate::fs::layout::Dir) -> Self::Output {
        todo!()
    }

    fn visit_inode_mut(&mut self, inode: &mut crate::fs::layout::INode) -> Self::Output {
        match &mut inode.node_type {
            NodeType::File(f) => f.accept_mut(self),
            NodeType::Dir(d) => d.accept_mut(self),
        }
    }

    fn visit_file_mut(&mut self, file: &mut crate::fs::layout::File) -> Self::Output {
        todo!()
    }

    fn visit_dir_mut(&mut self, dir: &mut crate::fs::layout::Dir) -> Self::Output {
        for child in dir.children.iter_mut() {
            match child.accept_mut(self) {
                Err(_) => {}
                Ok(()) => return Ok(()),
            }
        }
        Err(FSError::NodeNotFound(self.p.clone()))
    }
}

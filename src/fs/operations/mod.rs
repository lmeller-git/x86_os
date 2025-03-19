mod read_write;

use super::layout::{Dir, File, INode};

pub(in crate::fs) trait Visitable<V: Visitor>: Sized {
    fn accept(&self, visitor: &mut V) -> V::Output;
    fn accept_mut(&mut self, visitor: &mut V) -> V::Output;
}
pub(in crate::fs) trait Visitor: Sized {
    type Output;
    fn visit_inode(&mut self, inode: &INode) -> Self::Output;
    fn visit_file(&mut self, file: &File) -> Self::Output;
    fn visit_dir(&mut self, dir: &Dir) -> Self::Output;
    fn visit_inode_mut(&mut self, inode: &mut INode) -> Self::Output {
        self.visit_inode(inode)
    }
    fn visit_file_mut(&mut self, file: &mut File) -> Self::Output {
        self.visit_file(file)
    }
    fn visit_dir_mut(&mut self, dir: &mut Dir) -> Self::Output {
        self.visit_dir(dir)
    }
}

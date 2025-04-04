use alloc::{string::String, sync::Arc, vec::Vec};
use spin::mutex::Mutex;

use super::{FSError, Path, Result};

pub(super) struct Node {
    pub data: NodeData,
    pub name: String,
}

impl Node {
    pub fn dir(p: &str) -> Self {
        Self {
            data: NodeData::Dir(Dir::new()),
            name: p.into(),
        }
    }

    pub fn file(p: &str) -> Self {
        Self {
            data: NodeData::File(File::new()),
            name: p.into(),
        }
    }
    pub fn children(&self) -> Option<&[Arc<Mutex<Node>>]> {
        match &self.data {
            NodeData::Dir(d) => Some(&d.children),
            NodeData::File(_) => None,
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

pub(super) enum NodeData {
    Dir(Dir),
    File(File),
}

impl NodeData {
    pub fn file() -> Self {
        Self::File(File::new())
    }
    pub fn dir() -> Self {
        Self::Dir(Dir::new())
    }
}

pub(super) struct File {
    pub content: Vec<u8>,
}

impl File {
    pub fn new() -> Self {
        Self {
            content: Vec::new(),
        }
    }
}

pub(super) struct Dir {
    pub children: Vec<Arc<Mutex<Node>>>,
}

impl Dir {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

pub struct FS {
    root: Arc<Mutex<Node>>,
}

impl FS {
    pub(super) fn new() -> Self {
        Self {
            root: Arc::new(Mutex::new(Node::dir("/"))),
        }
    }

    pub(super) fn get(&self, p: &Path) -> Option<Arc<Mutex<Node>>> {
        let mut current = self.root.clone();
        if p.elements.is_empty() {
            return Some(current);
        }
        let mut path = p.elements.iter();
        #[allow(clippy::while_let_on_iterator)]
        'outer: while let Some(e) = path.next() {
            let borrow = current.lock();
            if let Some(children) = borrow.children() {
                for child in children {
                    if child.lock().name == *e {
                        let child = child.clone();
                        drop(borrow);
                        current = child;
                        continue 'outer;
                    }
                }
            }
            return None;
        }

        Some(current)
    }

    pub(super) fn read(&self, p: &Path) -> Result<Vec<u8>> {
        if let Some(node) = self.get(p) {
            match &node.lock().data {
                NodeData::Dir(_) => Err(FSError::NotImplemented("dir".into(), "read".into())),
                NodeData::File(f) => Ok(f.content.clone()),
            }
        } else {
            Err(FSError::NodeNotFound(p.into()))
        }
    }

    pub(super) fn write(&self, p: &Path, content: Vec<u8>) -> Result<()> {
        if let Some(node) = self.get(p) {
            match node.lock().data {
                NodeData::Dir(_) => Err(FSError::NotImplemented("dir".into(), "read".into())),
                NodeData::File(ref mut f) => {
                    f.content = content;
                    Ok(())
                }
            }
        } else {
            Err(FSError::NodeNotFound(p.into()))
        }
    }

    pub(super) fn insert(&self, p: &Path, node: NodeData) -> Result<()> {
        let (clipped_path, name) = p.split_last();
        if let Some(parent) = self.get(&clipped_path) {
            match parent.lock().data {
                NodeData::Dir(ref mut d) => {
                    if d.children.iter().any(|c| c.lock().name == name[0]) {
                        return Err(FSError::NodeExists(p.into()));
                    }
                    d.children.push(Arc::new(Mutex::new(Node {
                        data: node,
                        name: name[0].into(),
                    })));
                    Ok(())
                }
                NodeData::File(_) => Err(FSError::NodeExists(clipped_path.into())),
            }
        } else {
            Err(FSError::NodeNotFound(clipped_path.into()))
        }
    }

    pub(super) fn delete(&self, p: &Path) -> Result<()> {
        if p.elements.is_empty() {
            return Err(FSError::NotImplemented("root".into(), "delete".into()));
        }
        let (clipped_path, name) = p.split_last();
        if let Some(parent) = self.get(&clipped_path) {
            match parent.lock().data {
                NodeData::File(_) => Err(FSError::NodeNotFound(p.into())),
                NodeData::Dir(ref mut d) => {
                    for (i, child) in d.children.iter().enumerate() {
                        if child.lock().name == name[0] {
                            d.children.remove(i);
                            return Ok(());
                        }
                    }
                    Err(FSError::NodeNotFound(p.into()))
                }
            }
        } else {
            Err(FSError::NodeNotFound(p.into()))
        }
    }
}

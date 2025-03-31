//TODO make async

use alloc::{string::String, sync::Arc, vec::Vec};
use lazy_static::lazy_static;
use spin::mutex::Mutex;

use crate::fs::layout::FS;

use super::{
    layout::{Node, NodeData},
    FSError, Path, Result,
};

pub fn read(p: &Path) -> Result<Vec<u8>> {
    FILESYSTEM.lock().read(p)
}

pub fn write(p: &Path, content: Vec<u8>) -> Result<()> {
    FILESYSTEM.lock().write(p, content)
}

pub fn remove(p: &Path) -> Result<()> {
    FILESYSTEM.lock().delete(p)
}

pub fn insert_file(p: &Path) -> Result<()> {
    FILESYSTEM.lock().insert(p, NodeData::file())
}

pub fn insert_dir(p: &Path) -> Result<()> {
    FILESYSTEM.lock().insert(p, NodeData::dir())
}

pub struct NodeInfo {
    pub children: Vec<String>,
    pub name: String,
}

pub fn get_node_info(p: &Path) -> Result<NodeInfo> {
    if let Some(n) = FILESYSTEM.lock().get(p) {
        let children = match &n.lock().data {
            NodeData::Dir(d) => d.children.iter().map(|c| c.lock().name().into()).collect(),
            NodeData::File(_f) => Vec::new(),
        };
        let data = NodeInfo {
            children,
            name: n.lock().name().into(),
        };
        Ok(data)
    } else {
        Err(FSError::NodeNotFound(p.into()))
    }
}

lazy_static! {
    pub static ref FILESYSTEM: Mutex<FS> = Mutex::new(FS::new());
}

pub fn init() {
    let p = Path::new("/test");
    let content = (b"hello world").to_vec();
    insert_file(&p).expect("test file save failed");
    write(&p, content.clone()).expect("write to test file failed");
    assert_eq!(read(&p).expect("read from test file failed"), content);
    remove(&p).expect("removal of test file failed");
}

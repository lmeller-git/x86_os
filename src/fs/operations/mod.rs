//TODO make async

use alloc::vec::Vec;
use lazy_static::lazy_static;
use spin::mutex::Mutex;

use crate::fs::layout::FS;

use super::{layout::NodeData, Path, Result};

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

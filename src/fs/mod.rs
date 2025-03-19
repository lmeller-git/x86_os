use layout::Path;
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

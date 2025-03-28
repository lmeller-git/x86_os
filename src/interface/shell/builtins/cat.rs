use crate::{
    fs::{Path, PathBuf},
    interface::shell::{parse::Args, OutStream},
};

pub async fn cat<O: OutStream>(p: PathBuf, ostream: O, args: Args) {}

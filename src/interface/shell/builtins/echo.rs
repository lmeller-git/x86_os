use crate::{
    fs::Path,
    interface::shell::{parse::Args, OutStream},
};

pub async fn echo<O: OutStream>(ostream: O, args: Args) {}

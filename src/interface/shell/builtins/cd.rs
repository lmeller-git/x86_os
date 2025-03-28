use crate::{
    fs::Path,
    interface::shell::{parse::Args, OutStream},
};

pub async fn cd<O: OutStream>(ostream: O, args: Args) {}

use alloc::vec::Vec;

use crate::{
    fs::Path,
    interface::shell::{parse::Args, utils::ArgParser, OutStream},
};

pub async fn echo<O: OutStream>(ostream: O, args: Args) {
    if !args.named.is_empty() {
        todo!()
    }
    ostream.write(&args.positional.join(" "));
}

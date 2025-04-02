use alloc::{boxed::Box, vec::Vec};

use crate::{
    fs::Path,
    interface::shell::{parse::Args, utils::ArgParser, OutStream},
};

pub async fn echo(mut ostream: Box<dyn OutStream>, args: Args) {
    if !args.named.is_empty() {
        todo!()
    }
    ostream.write(&args.positional.join(" "));
}

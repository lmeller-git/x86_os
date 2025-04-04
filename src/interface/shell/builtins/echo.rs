use alloc::boxed::Box;

use crate::interface::shell::{parse::Args, OutStream};

pub async fn echo(mut ostream: Box<dyn OutStream>, mut _errstream: Box<dyn OutStream>, args: Args) {
    if !args.named.is_empty() {
        ostream.write("");
    }
    ostream.write(&args.positional.join(" "));
}

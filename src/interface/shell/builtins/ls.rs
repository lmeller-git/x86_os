use alloc::vec::Vec;

use crate::{
    fs::{self, Path, PathBuf},
    interface::shell::{parse::Args, utils::ArgParser, OutStream},
    println,
};

pub async fn ls<O: OutStream>(p: PathBuf, mut ostream: O, args: Args) {
    // let args: ArgParser<0> = ArgParser::try_from(args).unwrap();
    // println!("fine1");
    // let args = args.validate(Vec::new()).unwrap();
    // println!("fine");
    let res = fs::get_node_info(&p.as_path()).unwrap();
    // println!("fine2");
    for c in &res.children {
        ostream.write(c);
        ostream.write(" ");
    }
    ostream.write("\n");
}

use alloc::vec;

use crate::{
    fs::{self, Path, PathBuf},
    interface::shell::{parse::Args, utils::ArgParser, OutStream},
};

pub async fn rm<O: OutStream>(p: PathBuf, mut ostream: O, args: Args) {
    let args: ArgParser<1> = ArgParser::try_from(args)
        .unwrap()
        .validate(vec!["-r", "-f"])
        .unwrap();
    let p2 = Path::new(&args.positionals[0]);
    let p3 = p.as_path();
    let p = Path::try_concat(&p3, &p2);
    let res = fs::remove(&p);
}

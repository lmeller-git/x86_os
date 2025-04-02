use alloc::{boxed::Box, format, string::String, vec::Vec};

use crate::{
    fs::{self, Path, PathBuf},
    interface::shell::{parse::Args, utils::ArgParser, OutStream},
};

pub async fn cat(p: PathBuf, mut ostream: Box<dyn OutStream>, args: Args) {
    let args: ArgParser<1> = ArgParser::try_from(args)
        .unwrap()
        .validate(Vec::new())
        .unwrap();
    let p2 = Path::new(&args.positionals[0]);
    let p3 = p.as_path();
    let p = Path::try_concat(&p3, &p2);
    let res = fs::read(&p).unwrap();
    let res = res
        .iter()
        .map(|n| char::from_u32(*n as u32).unwrap())
        .collect::<String>();
    ostream.write(&res);
    ostream.write("\n");
}

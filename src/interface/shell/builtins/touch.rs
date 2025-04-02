use alloc::{boxed::Box, vec::Vec};

use crate::{
    fs::{self, Path, PathBuf},
    interface::shell::{
        parse::Args,
        utils::{ArgParseError, ArgParser},
        OutStream,
    },
    println,
};

pub async fn touch(p: PathBuf, ostream: Box<dyn OutStream>, args: Args) {
    let args: ArgParser<1> = ArgParser::try_from(args)
        .unwrap()
        .validate(Vec::new())
        .unwrap();
    let p2 = Path::new(&args.positionals[0]);
    let p3 = p.as_path();
    // println!("p2, p3: {}  {}", p2, p3);
    let p = Path::try_concat(&p3, &p2);
    // println!("p3: {}", p);
    // println!("p: {}", p);
    fs::insert_file(&p).unwrap();
}

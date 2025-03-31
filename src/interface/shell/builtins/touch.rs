use alloc::vec::Vec;

use crate::{
    fs::{self, Path, PathBuf},
    interface::shell::{
        parse::Args,
        utils::{ArgParseError, ArgParser},
        OutStream,
    },
};

pub async fn touch<O: OutStream>(p: PathBuf, ostream: O, args: Args) {
    let args: ArgParser<1> = ArgParser::try_from(args)
        .unwrap()
        .validate(Vec::new())
        .unwrap();
    let p = Path::try_concat(&p, &Path::new(args.positionals[0]));
    fs::insert_file(&p).unwrap();
}

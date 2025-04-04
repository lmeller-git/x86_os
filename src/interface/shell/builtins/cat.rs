use alloc::{boxed::Box, string::String, vec::Vec};

use crate::{
    fs::{self, Path, PathBuf},
    interface::shell::{parse::Args, utils::ArgParser, OutStream},
};

use super::validate_checked;

pub async fn cat(
    p: PathBuf,
    mut ostream: Box<dyn OutStream>,
    mut errstream: Box<dyn OutStream>,
    args: Args,
) {
    let args: ArgParser<'_, 1> = match validate_checked(args, Vec::new()) {
        Ok(v) => v,
        Err(e) => {
            _ = writeln!(errstream, "wrong arguments for cat: {}", e);
            return;
        }
    };
    let p2 = Path::new(&args.positionals[0]);
    let p3 = p.as_path();
    let p = Path::try_concat(&p3, &p2);
    let res = match fs::read(&p) {
        Ok(v) => v,
        Err(e) => {
            _ = writeln!(errstream, "cat failed: {}", e);
            return;
        }
    };
    let res = res
        .iter()
        .map(|n| char::from_u32(*n as u32).unwrap())
        .collect::<String>();
    ostream.write(&res);
    ostream.write("\n");
}

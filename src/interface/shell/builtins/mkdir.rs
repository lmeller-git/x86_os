use alloc::{boxed::Box, vec};

use crate::{
    fs::{self, Path, PathBuf},
    interface::shell::{parse::Args, utils::ArgParser, OutStream},
};

use super::validate_checked;

pub async fn mkdir(
    p: PathBuf,
    mut _ostream: Box<dyn OutStream>,
    mut errstream: Box<dyn OutStream>,
    args: Args,
) {
    let args: ArgParser<1> = match validate_checked(args, vec!["-p"]) {
        Ok(v) => v,
        Err(e) => {
            _ = writeln!(errstream, "wrong args for mkdir: {}", e);
            return;
        }
    };
    let p2 = Path::new(&args.positionals[0]);
    let p3 = p.as_path();
    let p = Path::try_concat(&p3, &p2);
    let e = fs::insert_dir(&p);
    if e.is_err() {
        _ = writeln!(errstream, "mkdir failed: {}", e.unwrap_err());
    }
}

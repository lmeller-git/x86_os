use alloc::boxed::Box;

use crate::{
    fs::{self, Path, PathBuf},
    interface::shell::{parse::Args, OutStream},
};

pub async fn ls(
    p: PathBuf,
    mut ostream: Box<dyn OutStream>,
    mut errstream: Box<dyn OutStream>,
    args: Args,
) {
    let p = if let Some(p2) = args.positional.first() {
        let p2 = Path::new(p2);
        let p3 = p.as_path();
        let p = Path::try_concat(&p3, &p2);
        p.into()
    } else {
        p
    };
    let res = match fs::get_node_info(&p.as_path()) {
        Ok(v) => v,
        Err(e) => {
            _ = writeln!(errstream, "ls failed: {}", e);
            return;
        }
    };

    for c in &res.children {
        ostream.write(c);
        ostream.write(" ");
    }
    ostream.write("\n");
}

use alloc::boxed::Box;

use crate::{
    exit_qemu,
    fs::Path,
    interface::shell::{parse::Args, OutStream},
    QemuExitCode,
};

pub async fn exit(ostream: Box<dyn OutStream>, args: Args) {
    exit_qemu(QemuExitCode::Success);
}

use alloc::boxed::Box;

use crate::{
    exit_qemu,
    interface::shell::{parse::Args, OutStream},
    QemuExitCode,
};

pub async fn exit(
    mut _ostream: Box<dyn OutStream>,
    mut _errstream: Box<dyn OutStream>,
    _args: Args,
) {
    exit_qemu(QemuExitCode::Success);
}

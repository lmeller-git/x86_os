use crate::{
    exit_qemu,
    fs::Path,
    interface::shell::{parse::Args, OutStream},
    QemuExitCode,
};

pub async fn exit<O: OutStream>(ostream: O, args: Args) {
    exit_qemu(QemuExitCode::Success);
}

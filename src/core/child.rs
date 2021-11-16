use std::process::{Command, ExitStatus};
use nix::unistd::{chroot, sethostname, getpid};
use nix::mount;
use std::env;

use super::{
    filesystem,
    namespace,
};


static DIR: &str = "/home/dezi/alpine-fs";

pub fn clone_child() -> isize {
    println!("Running as [{}]", getpid());

    namespace::create_isolated_namespace();
    sethostname("CONTAINER");

    filesystem::mount_rootfs(DIR);
    filesystem::set_root_fs(DIR);
    filesystem::mount_proc();


    let exits_status: ExitStatus = Command::new("sh")
        .spawn().expect("Failed to execute container command").wait().unwrap();
    let ex_code = match exits_status.code() {
        Some(code) => code as isize,
        None => -1
    };

    filesystem::unmount_proc();
    ex_code
}

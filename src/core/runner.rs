use clap::{App, Arg, SubCommand};
use nix::sched::{setns, clone, CloneFlags};
use nix::sys::signal::Signal;
use nix::sys::wait::{waitpid, WaitStatus};
use nix::unistd::sethostname;
use std::process::{Command, ExitStatus};

use super::child;

pub fn run() -> nix::Result<WaitStatus> {
    const STACK_SIZE: usize = 4 * 1024 * 1024; // 4 MB
    let ref mut stack: [u8; STACK_SIZE] = [0; STACK_SIZE];

    let cb = Box::new(|| child::clone_child());

    let clone_flags = CloneFlags::CLONE_NEWNS | CloneFlags::CLONE_NEWUTS |
        CloneFlags::CLONE_NEWPID | CloneFlags::CLONE_NEWIPC | CloneFlags::CLONE_NEWNET;
    let child_pid = clone(cb, stack, clone_flags, Some(Signal::SIGCHLD as i32))
        .expect("Failed to create child process");
    waitpid(child_pid, None)
}

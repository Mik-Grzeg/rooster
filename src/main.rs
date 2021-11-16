use clap::{App, Arg, SubCommand};
use nix::sched::{clone, CloneFlags, setns};
use nix::sys::signal::Signal;
use nix::sys::wait::{waitpid, WaitStatus};
use nix::unistd::sethostname;
use std::process::{Command, ExitStatus};

mod core;


fn main() {
    let matches = App::new( "UnixSocketClientServer" )
        .subcommand( SubCommand::with_name("run") )
        .subcommand( SubCommand::with_name("child"))
        .get_matches();

    match matches.subcommand() {
        ("run", Some(m)) => {core::runner::run();},
        ("child", Some(m)) => println!("Client"),
        _ => eprintln!("Wrong command"),
    };
}

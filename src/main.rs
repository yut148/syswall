mod child_process;
mod cli;
mod syscalls;

use std::env;
use nix::sys::ptrace;
use nix::unistd;

// NOTE: syscalls on Linux x86_64 use registers in the following order for arguments: rdi, rsi,
// rdx, r10, r8, r9

fn main() {
    let mut args = env::args();
    if args.len() <= 1 {
        eprintln!("Please specify an executable to run");
        return;
    }

    let child_cmd = match args.nth(1) {
        None => {
            eprintln!("Unable to get name of child process to execute");
            return;
        },
        Some(s) => s,
    };

    match unistd::fork().expect("unable to fork") {
        unistd::ForkResult::Parent{ child } => {
            eprintln!("Tracing child process {} ({})", child, child_cmd);

            // Wait for child and set trace options
            child_process::wait_child(child);
            ptrace::setoptions(child, ptrace::Options::PTRACE_O_EXITKILL).expect("Unable to set PTRACE_O_EXITKILL option");

            // Execute syscall wait loop
            child_process::child_loop(child);
        },
        unistd::ForkResult::Child => {
            child_process::exec_child(child_cmd, args);
        },
    };
}

use anyhow::Result;
use command_line_rust::strace::SystemCall;
use nix::{
    sys::{ptrace, wait::waitpid},
    unistd::Pid,
};
use owo_colors::OwoColorize;
use std::{os::unix::process::CommandExt, process::Command};

fn main() -> Result<()> {
    let arguments: Vec<_> = std::env::args().collect();
    let first_arg = &arguments[1];
    let mut command = Command::new(first_arg);
    command.args(&arguments[2..]);
    unsafe {
        command.pre_exec(|| ptrace::traceme().map_err(|e| e.into()));
    }

    let child = command.spawn()?;
    let child_pid = Pid::from_raw(child.id() as _);
    let res = waitpid(child_pid, None)?;
    eprintln!("first wait: {:?}", res.yellow());

    let mut is_sys_exit = false;
    loop {
        ptrace::syscall(child_pid, None)?;
        waitpid(child_pid, None)?;
        if is_sys_exit {
            let regs = ptrace::getregs(child_pid)?;
            let syscall_name: SystemCall = regs.orig_rax.try_into()?;
            eprintln!(
                "{:?}({:x}, {:x}, {:x}, ...) = {:x}",
                syscall_name.green(),
                regs.rdi.blue(),
                regs.rsi.blue(),
                regs.rdx.blue(),
                regs.rax.yellow(),
            );
        }
        is_sys_exit = !is_sys_exit;
    }
}

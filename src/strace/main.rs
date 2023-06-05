#[cfg(not(all(target_os = "linux", target_arch = "x86_64")))]
compile_error!("Not a supported platform");

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
use {
    anyhow::Result,
    command_line_rust::strace::SystemCall,
    nix::{
        sys::{ptrace, wait::waitpid},
        unistd::Pid,
    },
    owo_colors::OwoColorize,
    std::{os::unix::process::CommandExt, process::Command},
};

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
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

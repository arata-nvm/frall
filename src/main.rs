use std::{
    ffi,
    io::{self, Write},
};

use nix::{sys::wait, unistd};

#[allow(unreachable_code)]
fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().expect("failed to flush stdout");

        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("failed to read stdin");
        let command = line.trim();

        match unsafe { unistd::fork() } {
            Ok(unistd::ForkResult::Parent { child }) => {
                wait::waitpid(child, None).expect("failed to waitpid");
            }
            Ok(unistd::ForkResult::Child) => {
                let command_cstr = ffi::CString::new(command).expect("failed to CString::new");
                unistd::execvp(&command_cstr, &[command_cstr.clone()]).expect("failed to execvp");
            }
            Err(e) => panic!("failed to fork: {e}"),
        }
    }
}

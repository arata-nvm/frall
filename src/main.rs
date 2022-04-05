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
        let tokens: Vec<&str> = line.trim().split(" ").collect();

        match unsafe { unistd::fork() } {
            Ok(unistd::ForkResult::Parent { child }) => {
                wait::waitpid(child, None).expect("failed to waitpid");
            }
            Ok(unistd::ForkResult::Child) => {
                let tokens_cstr: Vec<ffi::CString> = tokens
                    .into_iter()
                    .map(|token| ffi::CString::new(token))
                    .collect::<Result<Vec<ffi::CString>, ffi::NulError>>()
                    .expect("failed to CString::new");
                unistd::execvp(&tokens_cstr[0], &tokens_cstr).expect("failed to execvp");
            }
            Err(e) => panic!("failed to fork: {e}"),
        }
    }
}

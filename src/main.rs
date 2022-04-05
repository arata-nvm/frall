use std::{
    ffi,
    io::{self, Write},
    path,
};

use nix::{sys::wait, unistd};

fn builtin_cd(args: &[&str]) {
    match args.len() {
        0 => unimplemented!(),
        1 => {
            let path = path::Path::new(&args[0]);
            if let Err(_) = unistd::chdir(path) {
                eprintln!("no such file or directory");
            }
        }
        _ => eprintln!("too many arguments"),
    }
}

fn exec_cmd(cmd: &str, args: &[&str]) {
    match unsafe { unistd::fork() } {
        Ok(unistd::ForkResult::Parent { child }) => {
            wait::waitpid(child, None).expect("failed to waitpid");
        }
        Ok(unistd::ForkResult::Child) => {
            let cmd_cstr = ffi::CString::new(cmd).expect("failed to CString::new");
            let mut args_cstr: Vec<ffi::CString> = args
                .iter()
                .map(|&arg| ffi::CString::new(arg))
                .collect::<Result<Vec<ffi::CString>, ffi::NulError>>()
                .expect("failed to CString::new");
            args_cstr.insert(0, cmd_cstr.clone());

            unistd::execvp(&cmd_cstr, &args_cstr).expect("failed to execvp");
        }
        Err(e) => panic!("failed to fork: {e}"),
    }
}

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

        let cmd = tokens[0];
        let args = &tokens[1..];
        match cmd {
            "cd" => builtin_cd(args),
            _ => exec_cmd(cmd, args),
        }
    }
}

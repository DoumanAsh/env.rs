#![no_main]

use std::{io, env, fs};
use std::io::Write;
use std::process::Command;

mod cli;

c_ffi::c_main!(run);

fn run(args: c_ffi::Args) -> std::os::raw::c_int {
    let args = match cli::Cli::parse(args.into_iter().skip(1)) {
        Ok(args) => args,
        Err(code) => return code
    };

    let mut cmd = None;
    let mut cmd_args = Vec::new();
    let mut extra_env = Vec::new();

    for arg in args.args.iter() {
        let mut split = arg.as_str().splitn(2, '=');
        let left = split.next().unwrap();

        if let Some(right) = split.next() {
            extra_env.push((left, right));
        } else if cmd.is_none() {
            cmd = Some(Command::new(arg))
        } else {
            cmd_args.push(arg.as_str());
        }
    }

    if let Some(mut cmd) = cmd {
        if let Some(chdir) = args.chdir {
            match fs::canonicalize(&chdir) {
                Ok(chdir) => {
                    cmd.current_dir(chdir);
                },
                Err(error) => {
                    eprintln!("{}: No such directory. Error: {}", chdir, error);
                    return 2;
                }
            }
        }

        if args.ignore_env {
            cmd.env_clear();
        }

        for name in args.unset.iter() {
            cmd.env_remove(name);
        }

        cmd.envs(extra_env);
        cmd.args(cmd_args);

        match cmd.status() {
            Ok(result) => result.code().unwrap_or(std::os::raw::c_int::max_value()),
            Err(error) => {
                eprintln!("Unable to start command. Error: {}", error);
                2
            }
        }
    } else {
        let stdout = io::stdout();
        let mut stdout = stdout.lock();

        if !args.ignore_env {
            for (name, value) in env::vars() {
                if !args.unset.contains(&name) {
                    let _ = write!(stdout, "{}={}\n", name, value);
                }
            }
        }

        for (name, value) in extra_env.iter() {
            let _ = write!(stdout, "{}={}\n", name, value);
        }

        0
    }
}

# env

![Build](https://github.com/DoumanAsh/env.rs/workflows/Rust/badge.svg?branch=master)
[![Crates.io](https://img.shields.io/crates/v/env-cli.svg)](https://crates.io/crates/env-cli)

Limited port of [env](http://man7.org/linux/man-pages/man1/env.1.html) utility

## Usage

```
env 1.0.1
Executes program in a modified environment

USAGE: [OPTIONS] [NAME=VALUE]... [COMMAND [ARG]...]

OPTIONS:
    -h,  --help                Prints this help information
    -i,  --ignore-environment  Start with an empty environment
    -C,  --chdir <chdir>       Changes working directory to specified one.
    -u,  --unset <unset>       Remove variable from the environment
```

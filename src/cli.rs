use core::fmt;

///env
///Executes program in a modified environment
pub struct Cli {
    ///Start with an empty environment
    pub ignore_env: bool,
    ///Changes working directory to specified one.
    pub chdir: Option<String>,
    ///Remove variable from the environment
    pub unset: Vec<String>,
    ///Environment maps(NAME=VALUE) and command
    pub args: Vec<String>,
}

pub struct Help {
    name: &'static str,
    version: &'static str,
    text: &'static str,
}

impl Help {
    const fn new(text: &'static str) -> Self {
        Self {
            name: "env",
            version: env!("CARGO_PKG_VERSION"),
            text,
        }
    }
}

impl fmt::Display for Help {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.name)?;
        f.write_str(" ")?;
        f.write_str(self.version)?;
        f.write_str("\n")?;
        f.write_str(self.text)
    }
}

const HELP_TEXT: &str = "Executes program in a modified environment

USAGE: [OPTIONS] [NAME=VALUE]... [COMMAND [ARG]...]

OPTIONS:
    -h,  --help                Prints this help information
    -i,  --ignore-environment  Start with an empty environment
    -C,  --chdir <chdir>       Changes working directory to specified one.
    -u,  --unset <unset>       Remove variable from the environment
";

const HELP: Help = Help::new(HELP_TEXT);

impl Cli {
    pub fn parse<'a, T: IntoIterator<Item = &'a str>>(args: T) -> Result<Self, std::os::raw::c_int> {
        let mut arguments = args.into_iter();

        let mut options_end = false;

        let mut ignore_env = false;
        let mut chdir = None;
        let mut unset = Vec::new();
        let mut args = Vec::new();

        while let Some(arg) = arguments.next() {
            if !options_end && arg.starts_with('-') {
                match &arg[1..] {
                    "h" | "-help" => {
                        println!("{}", HELP);
                        return Err(0);
                    },
                    "i" | "ignore-environment" => {
                        ignore_env = true;
                    },
                    "C" | "-chdir" => match arguments.next() {
                        Some(dir) => {
                            chdir = Some(dir.to_owned());
                        },
                        None => {
                            eprintln!("Specified option '{}' without value.", arg);
                            return Err(1);
                        },
                    },
                    "u" | "-unset" => match arguments.next() {
                        Some(name) => {
                            unset.push(name.to_owned());
                        },
                        None => {
                            eprintln!("Specified option '{}' without value.", arg);
                            return Err(1);
                        },
                    },
                    _ => {
                        eprintln!("Invalid option '{}' is specified.", arg);
                        return Err(1);
                    }
                }
            } else {
                options_end = true;
                args.push(arg.to_owned());
            }
        }

        Ok(Self {
            ignore_env,
            chdir,
            unset,
            args,
        })
    }
}

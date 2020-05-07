use arg::Args;

#[derive(Args, Debug)]
///env
///Executes program in a modified environment
pub struct Cli {
    #[arg(short, long = "ignore-environment")]
    ///Start with an empty environment
    pub ignore_env: bool,
    #[arg(short = "C", long)]
    ///Changes working directory to specified one.
    pub chdir: Option<String>,
    #[arg(short, long)]
    ///Remove variable from the environment
    pub unset: Vec<String>,
    ///Environment maps(NAME=VALUE) and command
    pub args: Vec<String>,
}

impl Cli {
    pub fn parse<'a, T: IntoIterator<Item = &'a str>>(args: T) -> Result<Self, std::os::raw::c_int> {
        let args = args.into_iter();
        Cli::from_args(args).map_err(|err| {
            println!("{}", err);
            !err.is_help() as std::os::raw::c_int
        })
    }
}

pico_args_helpgen::define_app! {
    app_name: "App name",
    app_description: "App description",
    app_version: "v0.1.0",

    help_args: "-h, --help",
    version_args: "-V, --version",

    struct AppArgs {
        subcommand: Option<String>, "new, add", "The subcommand to execute.",
        number: u32, "-n, --number", "The number to use in some operation.",
        flag: bool, "--flag", "A flag to enable or disable a feature.",
        freestanding: String, "", "A freestanding string argument.",
    }
}

fn parse_args() -> Result<AppArgs, pico_args_helpgen::Error> {
    let mut pargs = pico_args_helpgen::Arguments::from_env();

    handle_help_version(); // Important!

    let args = AppArgs {
        subcommand: pargs.subcommand()?,
        number: pargs.value_from_str(["-n", "--number"]).unwrap_or(10),
        flag: pargs.contains("--flag"),
        freestanding: pargs.free_from_str()?,
    };

    let remaining = pargs.finish();
    if !remaining.is_empty() {
        eprintln!("Unexpected argument(s): {:?}", remaining);
        std::process::exit(1);
    }

    Ok(args)
}

fn main() {
    let args = parse_args().unwrap_or_else(|e| {
        eprintln!("Error: {e}");
        std::process::exit(1);
    });

    println!("{:#?}", args);
}

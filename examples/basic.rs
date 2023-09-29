#![allow(dead_code)]

use pico_args_helpgen::{define_app, gen_help};

define_app! {
    app_name: "App name",
    app_description: "App description",

    struct AppArgs {
        subcommand: Option<String>, "The subcommand to execute.",
        number: u32, "The number to use in some operation.",
        flag: bool, "A flag to enable or disable a feature.",
        freestanding: String, "A freestanding string argument.",
    }
}

fn parse_args() -> Result<AppArgs, pico_args_helpgen::Error> {
    let mut pargs = pico_args_helpgen::Arguments::from_env();

    if pargs.contains(["-h", "--help"]) {
        print!("{}", gen_help(AppArgs::info()));
        std::process::exit(0);
    }

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

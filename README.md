# pico-args-helpgen

Fork of pico-args with clap like help generation.

## Example

```rust
pico_args_helpgen::define_app! {
    app_name: "App name",
    app_description: "App description",
    app_version: "v0.1.0",

    help_args: "-h, --help",
    version_args: "-V, --version",

    struct AppArgs {
        number: u32, "-n, --number", "The number to use in some operation.",
        flag: bool, "--flag", "A flag to enable or disable a feature.",
    }
}

fn parse_args() -> Result<AppArgs, pico_args_helpgen::Error> {
    let mut pargs = pico_args_helpgen::Arguments::from_env();

    handle_help_version();

    let args = AppArgs {
        number: pargs.value_from_str(["-n", "--number"]).unwrap_or(10),
        flag: pargs.contains("--flag"),
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
```

## Build features

- `eq-separator`

  Allows parsing arguments separated by `=`<br/>
  This feature adds about 1KiB to the resulting binary

- `short-space-opt`

  Makes the space between short keys and their values optional (e.g. `-w10`)<br/>
  If `eq-separator` is enabled, then it takes precedence and the '=' is not included.<br/>
  If `eq-separator` is disabled, then `-K=value` gives an error instead of returning `"=value"`.<br/>
  The optional space is only applicable for short keys because `--keyvalue` would be ambiguous

- `combined-flags`

  Allows combination of flags, e.g. `-abc` instead of `-a -b -c`<br/>
  If `short-space-opt` or `eq-separator` are enabled, you must parse flags after values,
  to prevent ambiguities

## Limitations

The main fundamental limitation of `pico-args` is that it parses arguments in an arbitrary order.
This is because we have a sort of "streaming" API and we don't know all the keys/arguments
beforehand. This could lead to some unexpected behaviors.
Specifically, let's say you have a following arguments:

```
--arg1 --arg2 value
```

If your parser tries to parse `--arg1` as key-value first, than its value would be `--arg2`
and not `value`, because the parser simply takes the "next" argument.
A properer parser would knew that `--arg2` is a key and will return an error,
since the value is missing.

If your parser tries to parse `--arg2` as a flag first and then `--arg1` as key-value,
then its value would be `value`, because `--arg2` was already removed by the parser
and the arguments list looks like `--arg1 value` to the parser.

If such behavior is unacceptable to your application, then you have to use a more high-level
arguments parsing library.

## Alternatives

The core idea of `pico-args` is to provide some "sugar" for arguments parsing without
a lot of overhead (binary or compilation time wise).
There are no point in comparing parsing features since `pico-args` supports
only the bare minimum. [Here](https://github.com/rust-cli/argparse-benchmarks-rs)
is a great comparison of various arguments parsing libraries.

## License

MIT

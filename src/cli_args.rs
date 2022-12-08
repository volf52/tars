pub const HELP: &str = "\
Tar Rust

USAGE:
  tars [OPTIONS] INPUT_FILE

FLAGS:
  -h, --help            Prints help information

OPTIONS:
  --out-dir -o DEST     Set output directory (default: '.')

ARGS:
  <INPUT_FILE>
";

#[derive(Debug)]
pub struct Args {
    pub out_dir: std::path::PathBuf,
    pub input: std::path::PathBuf,
}

pub fn parse_args() -> Result<Args, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();

    if pargs.contains(["-h", "--help"]) {
        print!("{}", HELP);
        std::process::exit(0);
    }

    let out_dir = {
        if let Some(long) = pargs.opt_value_from_os_str("--out-dir", parse_path)? {
            long
        } else {
            pargs
                .opt_value_from_os_str("-o", parse_path)?
                .unwrap_or_else(|| {
                    std::env::current_dir().expect("Failed to get current directory")
                })
        }
    };

    let args = Args {
        out_dir,
        input: pargs.free_from_str()?,
    };

    let remaining = pargs.finish();

    if !remaining.is_empty() {
        eprintln!("Unrecognized arguments: {remaining:?}")
    }

    Ok(args)
}

fn parse_path(s: &std::ffi::OsStr) -> Result<std::path::PathBuf, &'static str> {
    Ok(s.into())
}

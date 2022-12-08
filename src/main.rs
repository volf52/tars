use flate2::read::GzDecoder;
use std::fs::File;
use tar::Archive;

mod cli_args;

fn main() -> Result<(), std::io::Error> {
    let args = match cli_args::parse_args() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {e}");
            print!("{}", cli_args::HELP);
            std::process::exit(1);
        }
    };

    let tar_gz = File::open(args.input)?;

    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);

    archive.unpack(args.out_dir)?;

    Ok(())
}

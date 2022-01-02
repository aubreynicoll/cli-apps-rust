use anyhow::{Context, Result};
use std::path::PathBuf;
use std::{fs, io};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::from_args();
    let content = fs::read_to_string(&args.path)
        .with_context(|| format!("Error opening file {:?}", &args.path))?;
    let mut write_stream = io::stdout();

    grrs::find_matches(&content, &args.pattern, &mut write_stream)?;

    Ok(())
}

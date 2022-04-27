use clap::Parser;
use std::fs::File;
use std::io::{self, Write, BufRead, Error};

#[derive(Parser)]
struct Cli {
    pattern: String,
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf
}

fn main() -> Result<(), Error>{

    // build stdout buffer.
    let stdout = io::stdout(); // get the global stdout entity
    let mut handle = io::BufWriter::new(stdout);

    // parse args.
    let args = Cli::parse();

    // open file, read to buffer.
    let file = File::open(&args.path)?;
    let buffer = io::BufReader::new(file);

    // iterate over content and write pattern matches to stdout buffer.
    for line in buffer.lines() {
        // create ref so that we can use the value again.
        let content = &line?;

        if content.contains(&args.pattern) {
            writeln!(handle, "{}", content)?;
        }
    }
    
    Ok(())
}
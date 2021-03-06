use clap::{App, AppSettings, Arg};
use std::fs::{File, OpenOptions};
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .setting(AppSettings::ColoredHelp)
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .help("Read from the specified json file, instead of stdin.")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("out")
                .help("Write to the specified location, instead of stdout.")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("mini")
                .short("m")
                .long("mini")
                .help("Remove formatting whitespace from the output.")
                .takes_value(false)
                .required(false),
        )
        .get_matches();

    // TODO: Need a better way than boxing these.
    let input: Box<dyn io::Read> = match matches.value_of("file") {
        Some(f) => Box::new(io::BufReader::new(File::open(f)?)),
        None => Box::new(io::stdin()),
    };

    let output: Box<dyn io::Write> = match matches.value_of("output") {
        Some(o) => Box::new(io::BufWriter::new(
            OpenOptions::new().create(true).truncate(true).open(o)?,
        )),
        None => Box::new(io::stdout()),
    };

    let f: serde_json::value::Value = serde_json::from_reader(input)?;
    if matches.is_present("mini") {
        serde_json::to_writer(output, &f)?;
    } else {
        serde_json::to_writer_pretty(output, &f)?;
    }

    Ok(())
}

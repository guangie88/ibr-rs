extern crate failure;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;
#[macro_use]
extern crate vlog;

use std::fs::read_dir;
use std::path::PathBuf;
use structopt::StructOpt;

type Result<T> = std::result::Result<T, failure::Error>;

#[derive(StructOpt, Debug)]
#[structopt(name = "line-stickers-scraper-conf")]
/// Configuration for line-stickers-scraper
struct Conf {
    #[structopt(short = "i", long = "indir", parse(from_os_str))]
    /// Input directory with the image files
    indir: PathBuf,

    #[structopt(short = "v", parse(from_occurrences))]
    /// Verbose flag (-v, -vv, -vvv)
    verbose: u8,
}

fn run(conf: &Conf) -> Result<()> {
    vlog::set_verbosity_level(conf.verbose as usize);

    let paths = read_dir(&conf.indir)?;

    let mut paths: Vec<_> = paths
        .filter_map(|path| path.map(|path| path.path()).ok())
        .collect();

    paths.sort();

    for path in paths {
        v0!("{:?}", path);
    }

    Ok(())
}

fn main() {
    let conf = Conf::from_args();

    match run(&conf) {
        Ok(_) => v1!("ibr COMPLETED!"),
        Err(e) => ve0!("{}", e),
    }
}

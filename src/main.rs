#![cfg_attr(feature = "cargo-clippy", deny(clippy))]
#![deny(missing_debug_implementations, warnings)]

#[macro_use]
extern crate failure;
extern crate glob;
extern crate image;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;
#[macro_use]
extern crate vlog;

use glob::glob;
use image::FilterType;
use std::fs::create_dir_all;
use std::path::PathBuf;
use structopt::StructOpt;

type Result<T> = std::result::Result<T, failure::Error>;

#[derive(StructOpt, Debug)]
#[structopt(name = "image-batch-resizer")]
/// Configuration for image-batch-resizer
struct Conf {
    #[structopt(parse(from_os_str))]
    /// Input directory with the image files
    indir: PathBuf,

    #[structopt(short = "d", long = "delete")]
    /// Delete original image file that has been successfully resized
    delete: bool,

    #[structopt(
        short = "o",
        long = "outdir",
        default_value = "resized",
        parse(from_os_str)
    )]
    /// Only used when delete is false. Directory path to append to given `indir`
    /// path when saving the resized image files.
    outdir: PathBuf,

    #[structopt(short = "g", long = "glob", default_value = "*")]
    /// Glob pattern whose files must match in `indir`.
    glob: String,

    #[structopt(short = "m", long = "max")]
    /// Finds the maximum side (between height and width) and resize that side
    /// to this value proportionally
    max_side: u32,

    #[structopt(short = "v", parse(from_occurrences))]
    /// Verbose flag (-v, -vv, -vvv)
    verbose: u8,
}

macro_rules! cont_log_err {
    ($expr:expr, $fmt:expr) => {
        match $expr {
            Ok(o) => o,
            Err(e) => {
                ve0!($fmt, e);
                continue;
            }
        }
    };
}

macro_rules! cont_log_opt {
    ($expr:expr, $msg:expr) => {
        match $expr {
            Some(o) => o,
            None => {
                ve0!("{}", $msg);
                continue;
            }
        }
    };
}

fn run(conf: &Conf) -> Result<()> {
    vlog::set_verbosity_level(conf.verbose as usize);

    if !conf.indir.exists() {
        Err(format_err!(
            "{:?} input directory does not exists!",
            &conf.indir
        ))?
    }

    let merged_glob = {
        let mut indir = conf.indir.clone();
        indir.push(&conf.glob);
        indir.to_string_lossy().to_string()
    };

    // gets all the image files
    let paths = glob(&merged_glob)?;

    let mut paths: Vec<_> = paths
        .filter_map(|path| path.ok())
        .filter(|path| !path.is_dir())
        .collect();

    paths.sort();

    // create the output directory to hold resized images
    let outdir = {
        let mut outdir = conf.indir.clone();
        outdir.push(&conf.outdir);
        outdir
    };

    if !conf.delete {
        create_dir_all(&outdir)?;
    }

    for path in paths {
        let im = cont_log_err!(image::open(&path), "Error opening image: {}");

        let resized_im = im.resize(
            conf.max_side,
            conf.max_side,
            FilterType::Lanczos3,
        );

        let resized_path = if conf.delete {
            v2!("Resizing {:?}...", path);
            path
        } else {
            let file_name = cont_log_opt!(
                path.file_name(),
                format!(
                    "{:?} unexpectedly does not have file stem!",
                    path
                )
            );

            let mut resized_path = outdir.clone();
            resized_path.push(&file_name);

            v2!("Resizing {:?} -> {:?}...", path, resized_path);
            resized_path
        };

        cont_log_err!(
            resized_im.save(&resized_path),
            "Error saving image: {}"
        );
    }

    Ok(())
}

fn main() {
    let conf = Conf::from_args();

    match run(&conf) {
        Ok(_) => v1!("image-batch-resizer COMPLETED!"),
        Err(e) => ve0!("{}", e),
    }
}

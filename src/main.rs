#![cfg_attr(feature = "cargo-clippy", deny(clippy))]
#![deny(missing_debug_implementations, warnings)]

extern crate failure;
extern crate image;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;
#[macro_use]
extern crate vlog;

use image::FilterType;
use std::fs::read_dir;
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

    #[structopt(short = "s", long = "suffix", default_value = "_resized")]
    /// Only used when delete is false. Suffix to append to the resized image
    /// file name.
    suffix: String,

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

    let paths = read_dir(&conf.indir)?;

    let mut paths: Vec<_> = paths
        .filter_map(|path| path.map(|path| path.path()).ok())
        .collect();

    paths.sort();

    for path in paths {
        v2!("Resizing {:?}...", path);

        let im = cont_log_err!(image::open(&path), "Error opening image: {}");

        let resized_im = im.resize(
            conf.max_side,
            conf.max_side,
            FilterType::Lanczos3,
        );

        let resized_path = if conf.delete {
            path
        } else {
            let file_stem = cont_log_opt!(
                path.file_stem(),
                format!(
                    "{:?} unexpectedly does not have file stem!",
                    path
                )
            );

            let ext = cont_log_opt!(
                path.extension(),
                format!(
                    "{:?} unexpectedly does not have extension!",
                    path
                )
            );

            let mut new_path = path.clone();

            let mut file_name = file_stem.to_os_string();
            file_name.push(&conf.suffix);
            file_name.push(".");
            file_name.push(ext);

            new_path.set_file_name(&file_name);
            new_path
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

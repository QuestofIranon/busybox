use structopt::StructOpt;
use std::fs::File;
use std::io;

#[derive(StructOpt, Debug)]
#[structopt(name="touch", about ="change file access and modification times")]
struct Touch {
    /// Adjust the access and modification time stamps for the file by the specified value.  This flag is intended for use in modifying files with incorrectly set time stamps.
    //
    //             The argument is of the form ``[-][[hh]mm]SS'' where each pair of letters represents the following:
    //
    //                   -       Make the adjustment negative: the new time stamp is set to be before the old one.
    //                   hh      The number of hours, from 00 to 99.
    //                   mm      The number of minutes, from 00 to 59.
    //                   SS      The number of seconds, from 00 to 59.
    //
    //             The -A flag implies the -c flag: if any file specified does not exist, it will be silently ignored.
    #[structopt(short="A")]
    adjust: Option<String>,

    /// Change the access time of the file.  The modification time of the file is not changed unless the -m flag is also specified.
    #[structopt(short="a")]
    access: bool,

    /// Do not create the file if it does not exist.  The touch utility does not treat this as an error.  No error messages are displayed and the exit value is not affected.
    #[structopt(short="c")]
    no_create: bool,

    /// Attempt to force the update, even if the file permissions do not currently permit it.
    #[structopt(short="f")]
    force: bool,

    /// If the file is a symbolic link, change the times of the link itself rather than the file that the link points to.  Note that -h implies -c and thus will not create any new files.
    #[structopt(short="h")]
    links_only: bool,

    /// Change the modification time of the file.  The access time of the file is not changed unless the -a flag is also specified.
    #[structopt(short="m")]
    modification: bool,

    /// Use the access and modifications times from the specified file instead of the current time of day.
    #[structopt(short="r")]
    use_file: Option<String>,

    /// Change the access and modification times to the specified time instead of the current time of day.  The argument is of the form ``[[CC]YY]MMDDhhmm[.SS]'' where each pair of letters represents the following:
    //
    //                   CC      The first two digits of the year (the century).
    //                   YY      The second two digits of the year.  If ``YY'' is specified, but ``CC'' is not, a value for ``YY'' between 69 and 99 results in a
    //                           ``CC'' value of 19.  Otherwise, a ``CC'' value of 20 is used.
    //                   MM      The month of the year, from 01 to 12.
    //                   DD      the day of the month, from 01 to 31.
    //                   hh      The hour of the day, from 00 to 23.
    //                   mm      The minute of the hour, from 00 to 59.
    //                   SS      The second of the minute, from 00 to 61.
    //
    //             If the ``CC'' and ``YY'' letter pairs are not specified, the values default to the current year.  If the ``SS'' letter pair is not specified, the value defaults to 0.
    #[structopt(short="t")]
    specific_time: Option<String>,

    files: Vec<String>
}


fn main() -> io::Result<()> {
    // Arguments can be parsed using structopt as defined in the struct above
    // More information can be found in the structopt docs found at
    // https://docs.rs/structopt/0.2.14/structopt/
    let args = Touch::from_args();

    for file in args.files {
        let _ = File::create(file)?;
    }

    Ok(())
}

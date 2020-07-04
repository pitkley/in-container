// Copyright Pit Kleyersburg <pitkley@googlemail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified or distributed
// except according to those terms.

use clap::{crate_authors, crate_description, crate_name, crate_version, Clap};

#[derive(Clap)]
#[clap(
    author = crate_authors!(),
    version = crate_version!(),
    about = crate_description!(),
    after_help = concat!(
        "If no commands, options or parameters are specified, ", crate_name!(), " will only return \
         an exit-code after execution with no further output. This exit-code will be zero if ",
         crate_name!(), " was run within a container, and a non-zero exit-code (specifically 1) if \
         it was not run inside a container.\n\n\
         If ", crate_name!(), " encounters any errors or otherwise fails, it will try to exit with \
         exit-code 101."
    ),
)]
struct Opts {
    /// Specify if you want in-container to print the container-runtime it has detected. There will
    /// be no output if it is not running inside a container.
    #[clap(short = "p", long = "print")]
    print: bool,
    /// Do not exit with exit-code 1 if not running inside a container. (This can only be used if
    /// --print is specified.)
    #[clap(short = "d", long = "disable-exit-code", requires = "print")]
    disable_exit_code: bool,
}

fn main() {
    let opts = Opts::parse();
    let container_runtime = in_container::get_container_runtime();

    if opts.print {
        if let Some(container_runtime) = &container_runtime {
            println!("{}", container_runtime);
        }
    }

    if !opts.disable_exit_code && container_runtime.is_none() {
        std::process::exit(1);
    }
}

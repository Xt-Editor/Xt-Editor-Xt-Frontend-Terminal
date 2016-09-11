// Copyright 2016 Dom Rodriguez
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

extern crate rustc_serialize;
extern crate docopt;
extern crate slacstui;

#[macro_use]
extern crate log;
extern crate fern;
extern crate time;

use docopt::Docopt;

use slacstui::logging::logger::init_logger;

const USAGE: &'static str = "
slacs-tui

Usage:
  slacs-tui new [--nodaemon]
  slacs-tui (-h | --help)
  slacs-tui --version

Options:
  -h --help     Show this screen.
  --version     Show version.
  --nodaemon    Don't fork to background.
";

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_nodaemon: bool,
    flag_version: bool,
    cmd_new: bool,
}

fn print_version() {
    println!("slacs-tui version: {}", VERSION);
}

fn get_arguments() -> Args {
    Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit())
}

fn init() {
    // Initialise logging
    init_logger();
    trace!("Logging initialised.");
}

fn main() {
    init();

    trace!("Ascertain argument struct..");
    let args = get_arguments();

    info!("slacs-tui version: {} - starting NOW..", VERSION);

    if args.flag_version {
        trace!("Arguments tell me to print my version..");
        trace!("Complying, version printing now..");
        print_version();
        ::std::process::exit(0);
    }
}

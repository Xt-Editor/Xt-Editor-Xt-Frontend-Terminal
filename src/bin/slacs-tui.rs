// Copyright 2016 Dom Rodriguez
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

extern crate rustc_serialize;
extern crate docopt;
<<<<<<< HEAD
extern crate xtensistui;
=======
extern crate slacstui;
>>>>>>> gh/master
extern crate rustbox;
extern crate time;

#[macro_use]
extern crate log;
extern crate fern;

use std::error::Error;
use std::default::Default;

use rustbox::{Color, RustBox};
use rustbox::Key;

use docopt::Docopt;

use xtensistui::logging::logger::init_logger;

const USAGE: &'static str = "
xtensis-tui

Usage:
  xtensis-tui new [--nodaemon]
  xtensis-tui (-h | --help)
  xtensis-tui --version

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
    println!("xtensis-tui version: {}", VERSION);
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

    info!("xtensis-tui version: {} - starting NOW..", VERSION);

    if args.flag_version {
        trace!("Arguments tell me to print my version..");
        trace!("Complying, version printing now..");
        print_version();
        ::std::process::exit(0);
    }

    let rustBox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    rustBox.print(1,
                  1,
                  rustbox::RB_BOLD,
                  Color::White,
                  Color::Black,
<<<<<<< HEAD
                  "Welcome to xtensis.");
=======
                  "Welcome to slacs.");
>>>>>>> gh/master

    rustBox.print(1,
                  3,
                  rustbox::RB_BOLD,
                  Color::White,
                  Color::Black,
                  "Press `q` to quit.");

    rustBox.present();

    loop {
        rustBox.present();
        match rustBox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Char('q') => {
                        break;
                    }
                    _ => {}
                }
            }
            Err(e) => panic!("{}", e),
            _ => {}
        }
    }

}

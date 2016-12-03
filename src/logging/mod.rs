// This file is part of Xtensis.

// Xtensis is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Xtensis is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Xtensis.  If not, see <http://www.gnu.org/licenses/>

extern crate fern;
extern crate log;
extern crate time;

/// Initialises the global logger with the Fern backend.
pub fn init_logger() {
    let logging_config = fern::DispatchConfig {
        format: Box::new(|msg: &str, level: &log::LogLevel, _location: &log::LogLocation| {
            format!("[{}][{}] {}",
                    time::now().strftime("%Y-%m-%d][%H:%M:%S").unwrap(),
                    level,
                    msg)
        }),
        output: vec![fern::OutputConfig::stdout(), fern::OutputConfig::file("output.log")],
        level: log::LogLevelFilter::Trace,
    };

    if let Err(e) = fern::init_global_logger(logging_config, log::LogLevelFilter::Trace) {
        panic!("Failed to init global logger: {}", e)
    }
}

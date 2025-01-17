//! Watch files in a Cargo project and compile it when they change
#![forbid(unsafe_code, clippy::pedantic)]

extern crate cargo_watch;
extern crate watchexec;

fn main() -> watchexec::error::Result<()> {
    let matches = cargo_watch::args::parse();

    cargo_watch::change_dir();

    let quiet = matches.is_present("quiet");
    let debug = matches.is_present("debug");
    let opts = cargo_watch::get_options(debug, &matches);
    let handler = cargo_watch::watch::CwHandler::new(opts, quiet)?;
    watchexec::run::watch(&handler)
}

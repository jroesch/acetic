#![feature(env)]
#![feature(core)]
#![feature(os)]
extern crate acetic;
#[macro_use] extern crate log;

use std::env;

fn main() {
    for arg in env::args().skip(1) {
        debug!("Parsing: {:?}", arg);
        let krate = acetic::parse_crate(&arg.into_string().unwrap()[]);
        acetic::compile_crate(krate)
    }
}

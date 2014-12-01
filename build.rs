extern crate gcc;

use std::io::Command;
use std::os;
use std::default::Default;

fn main() {
    let out_dir = os::getenv("OUT_DIR").unwrap();
    gcc::compile_library("liblinenoise.a", &Default::default(),  &["src/linenoise.c"]);
}
extern crate gcc;

use std::io::Command;
use std::os;
use std::default::Default;

fn main() {
    let out_dir = os::getenv("OUT_DIR").unwrap();
    println!("Building Linenoise");

    // // note that there are a number of downsides to this approach, the comments
    // // below detail how to improve the portability of these commands.
    // Command::new("gcc").args(&["src/linenoise.c", "-Wall", "-W", "-Os", "-g", "-c", "-o"])
    //                    .arg(format!("{}/linenoise.o", out_dir))
    //                    .status().unwrap();
    // Command::new("ar").args(&["crus", "liblinenoise.a", "linenoise.o"])
    //                   .cwd(&Path::new(&out_dir))
    //                   .status().unwrap();

    // println!("cargo:rustc-flags=-L {} -l linenoise:static", out_dir);


    gcc::compile_library("liblinenoise.a", &Default::default(),  &["src/linenoise.c"]);

}
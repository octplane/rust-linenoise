extern crate gcc;

fn main() {
	gcc::compile_library("liblinenoise.a", &["native/linenoise.c"]);
}
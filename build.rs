extern crate gcc;

fn main() {
  gcc::Config::new()
    .cpp(true) // Switch to C++ library compilation.
    .file("native/wcwidth.cpp")
    .file("native/ConvertUTF.cpp")
    .file("native/linenoise.cpp")
    .include("native/")
    .compile("liblinenoise.a");
}

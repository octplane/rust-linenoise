extern crate cc;

fn main() {
  cc::Build::new()
    .cpp(true) // Switch to C++ library compilation.
    .file("native/wcwidth.cpp")
    .file("native/ConvertUTF.cpp")
    .file("native/linenoise.cpp")
    .flag("--std=c++0x") // char32_t support for mingw64
    .include("native/")
    .compile("liblinenoise.a");
}

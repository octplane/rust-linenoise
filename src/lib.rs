#![crate_name="linenoise"]
#![crate_type="lib"]

//! This is a library that interfaces with the linenoise library.
//! [Linenoise](https://github.com/antirez/linenoise) is a library implemented by Antirez, the Redis creator as a
//! replacement for readline.
//!
//! This library is just a binding to this library.
//!
//! ```rust
//! extern crate linenoise;
//! fn callback(input: &str) -> Vec<String> {
//!   let mut ret : Vec<&str>;
//!    if input.starts_with("s") {
//!     ret = vec!["suggestion", "suggestion2", "suggestion-three"];
//!   } else {
//!     ret = vec!["wot"];
//!   }
//!
//!     return ret.iter().map(|s| s.to_string()).collect();
//! }
//!
//! fn main() {
//!   linenoise::set_callback(callback);
//!
//!     loop {
//!       let val = linenoise::input("hello > ");
//!         match val {
//!             None => { break }
//!             Some(input) => {
//!                 println!("> {}", input);
//!                 linenoise::history_add(input.as_slice());
//!                 if input.as_slice() == "clear" {
//!                   linenoise::clear_screen();
//!                 }
//!             }
//!         }
//!     }
//! }
//! ```


extern crate libc;

use std::ffi::CString;
use std::ffi::CStr;
use std::str;

pub mod ffi;


pub type Completions = ffi::Struct_linenoiseCompletions;
type Callback = ffi::linenoiseCompletionCallback;

pub type CompletionCallback = fn(&str) -> Vec<String>;
static mut USER_COMPLETION: Option<CompletionCallback> = None;


/// Sets the callback when tab is pressed
pub fn set_callback(rust_cb: CompletionCallback ) {
    unsafe {
        USER_COMPLETION = Some(rust_cb);
        let ca = internal_callback as *mut _;
        ffi::linenoiseSetCompletionCallback(ca);
    }
}

/// Shows the prompt with your prompt as prefix
/// Retuns the typed string or None is nothing or EOF
pub fn input(prompt: &str) -> Option<String> {
    let cprompt = CString::new(prompt.as_bytes()).unwrap();

    unsafe {
        let cs = cprompt.as_ptr();
        let rret = ffi::linenoise(cs);

        let rval = if rret != 0 as *mut i8 {
            let ptr = rret as *const i8;
            let cast = str::from_utf8(CStr::from_ptr(ptr).to_bytes()).unwrap().to_string();
            libc::free(ptr as *mut libc::c_void);
            Some(cast)
        } else {
            None
        };
        return rval;
    }
}

/// Add this string to the history
pub fn history_add(line: &str) -> i32 {
    let cs = CString::new(line).unwrap().as_ptr();
    let ret: i32;
    unsafe {
        ret = ffi::linenoiseHistoryAdd(cs);
    }
    ret
}

/// Set max length history
pub fn history_set_max_len(len: i32) -> i32 {
    let ret: i32;
    unsafe {
        ret = ffi::linenoiseHistorySetMaxLen(len);
    }
    ret
}

/// Save the history on disk
pub fn history_save(file: &str) -> i32 {
    let fname = CString::new(file).unwrap().as_ptr();
    let ret: i32;
    unsafe {
        ret = ffi::linenoiseHistorySave(fname);
    }
    ret
}

/// Load the history on disk
pub fn history_load(file: &str) -> i32 {
    let fname = CString::new(file).unwrap().as_ptr();
    let ret: i32;
    unsafe {
        ret = ffi::linenoiseHistoryLoad(fname);
    }
    ret
}

/// Get a line from the history by (zero-based) index
pub fn history_line(index: i32) -> Option<String> {
    unsafe {
        let ret = ffi::linenoiseHistoryLine(index);
        let rval = if ret != 0 as *mut i8 {
            let ptr = ret as *const i8;
            let cast = str::from_utf8(CStr::from_ptr(ptr).to_bytes()).unwrap().to_string();
            libc::free(ptr as *mut libc::c_void);
            Some(cast)
        } else {
            None
        };
        return rval;
    }
}

///Clears the screen
pub fn clear_screen() {
    unsafe {
        ffi::linenoiseClearScreen();
    }
}

pub fn set_multiline(ml: i32) {
    unsafe {
        ffi::linenoiseSetMultiLine(ml);
    }
}

pub fn print_key_codes() {
    unsafe {
        ffi::linenoisePrintKeyCodes();
    }
}


/// Add a completion to the current list of completions.
pub fn add_completion(c: *mut Completions, s: &str) {
    unsafe {
        ffi::linenoiseAddCompletion(c, CString::new(s).unwrap().as_ptr());
    }
}

fn internal_callback(cs: *mut libc::c_char, lc:*mut Completions ) {
    unsafe {
        (*lc).len = 0;
    }
    let cr = cs as *const _;


    unsafe {
        let input = str::from_utf8(CStr::from_ptr(cr).to_bytes()).unwrap();
        for external_callback in USER_COMPLETION.iter() {
            let ret = (*external_callback)(input);
            for x in ret.iter() {
                add_completion(lc, x.as_ref());
            }
        }
    }
}

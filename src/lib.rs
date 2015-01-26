#![crate_type="lib"]
#![feature(slicing_syntax)]
#![feature(int_uint)]
#![allow(unstable)]

extern crate libc;

use std::ffi::CString;
pub mod ffi;


pub type Completions = ffi::Struct_linenoiseCompletions;
type Callback = ffi::linenoiseCompletionCallback;

pub type CompletionCallback = fn(&str) -> Vec<String>;
static mut USER_COMPLETION: Option<CompletionCallback> = None;

fn from_c_str<'a>(p: &'a *const libc::c_char) -> &'a str {
    std::str::from_utf8( unsafe { std::ffi::c_str_to_bytes(p) } ).ok().expect("Found invalid utf8")
}

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
    let cprompt = CString::from_slice(prompt.as_bytes());

    unsafe {
        let cs = cprompt.as_ptr();
        let rret = ffi::linenoise(cs);

        let rval = if rret != 0 as *mut i8 {
            let ptr = rret as *const i8;
            let cast = from_c_str(&ptr);
            Some(cast.to_string())
        } else {
            None
        };
        return rval;
    }
}

/// Add this string to the history
pub fn history_add(line: &str) -> i32 {
    let cs = CString::from_slice(line.as_bytes()).as_slice_with_nul().as_ptr();
    let mut ret: i32;
    unsafe {
        ret = ffi::linenoiseHistoryAdd(cs);
    }
    ret
}

/// Set max length history
pub fn history_set_max_len(len: i32) -> i32 {
    let mut ret: i32;
    unsafe {
        ret = ffi::linenoiseHistorySetMaxLen(len);
    }
    ret
}

/// Save the history on disk
pub fn history_save(file: &str) -> i32 {
    let fname = CString::from_slice(file.as_bytes()).as_slice_with_nul().as_ptr();
    let mut ret: i32;
    unsafe {
        ret = ffi::linenoiseHistorySave(fname);
    }
    ret
}

/// Load the history on disk
pub fn history_load(file: &str) -> i32 {
    let fname = CString::from_slice(file.as_bytes()).as_slice_with_nul().as_ptr();
    let mut ret: i32;
    unsafe {
        ret = ffi::linenoiseHistoryLoad(fname);
    }
    ret
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
        ffi::linenoiseAddCompletion(c, CString::from_slice(s.as_bytes()).as_slice_with_nul().as_ptr());
    }
}

fn internal_callback(cs: *mut libc::c_char, lc:*mut Completions ) {
    unsafe {
        (*lc).len = 0;
    }
    let input: &str;
    let cr = cs as *const _;

    input = from_c_str(&cr);

    unsafe {
        for external_callback in USER_COMPLETION.iter() {
            let ret = (*external_callback)(input);
            for x in ret.iter() {
                add_completion(lc, x.as_slice());
            }
        }
    }
}

extern crate libc;
use std::c_str;
pub mod ffi;

pub type Completions = ffi::Struct_linenoiseCompletions;

pub fn set_completion_callback(cb: fn(*mut libc::c_char, *mut Completions )) {
    unsafe {
        let ca = cb as *mut _;
        ffi::linenoiseSetCompletionCallback(ca);
    }
}

pub fn linenoise(prompt: &str) -> Option<String> {
    let cprompt = prompt.to_c_str();
    let mut retval:Option<String>;

    unsafe {
        let cs = cprompt.as_ptr();
        let rret = ffi::linenoise(cs);

        let ptr = rret as *const i8;
        let ret = c_str::CString::new(ptr, true);
        let cast = ret.as_str();

        match cast {
            None => { retval = None }
            _ => {
                let tmp = cast.unwrap ();
                retval = Some (tmp.to_string());
            }
        }
    }
    retval
}

pub fn history_add(line: &str) -> i32 {
    let cs = line.to_c_str().as_ptr();
    let mut ret: i32;
    unsafe {
        ret = ffi::linenoiseHistoryAdd(cs);
    }
    ret
}

pub fn history_set_max_len(len: i32) -> i32 {
    let mut ret: i32;
    unsafe {
        ret = ffi::linenoiseHistorySetMaxLen(len);
    }
    ret
}

pub fn history_save(file: &str) -> i32 {
    let fname = file.to_c_str().as_ptr();
    let mut ret: i32;
    unsafe {
        ret = ffi::linenoiseHistorySave(fname);
    }
    ret
}

pub fn history_load(file: &str) -> i32 {
    let fname = file.to_c_str().as_ptr();
    let mut ret: i32;
    unsafe {
        ret = ffi::linenoiseHistoryLoad(fname);
    }
    ret
}

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

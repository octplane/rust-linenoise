extern crate libc;
use std::c_str;
pub mod ffi;

type Completions = ffi::Struct_linenoiseCompletions;
type Callback = ffi::linenoiseCompletionCallback;

fn set_completion_callback(cb: fn(*mut libc::c_char, *mut Completions )) {
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

// Start the official external API
pub type CompletionCallback = fn(&str) -> Vec<&str>;
pub struct Linenoise;
impl Copy for Linenoise {}

pub static LINENOISE : Linenoise  = Linenoise;
static mut USER_COMPLETION : Option<CompletionCallback> = None;


impl Linenoise {
    pub fn init(&self, rust_cb: CompletionCallback ) {
        unsafe {
            USER_COMPLETION = Some(rust_cb);
        }

        set_completion_callback(Linenoise::internal_callback);
    }
    pub fn input(&self, prompt: &str) -> Option<String> {

        return linenoise(prompt);
    }

    fn internal_callback(cs: *mut libc::c_char, lc:*mut Completions ) {
        unsafe {
            (*lc).len = 0;
        }
        let input: Option<&str>;
        let ccurrent_input: std::c_str::CString;

        unsafe {
            ccurrent_input = c_str::CString::new(cs as *const _, false);
            input = ccurrent_input.as_str();
        }
        match input {
            None => { return; }
            Some(completable) => {
                unsafe {
                    match USER_COMPLETION {
                        None => { return; }
                        Some(external_callback) => {
                            println!("Calling callback");
                            let ret = external_callback(completable);
                            println!("ret= {}", ret);
                            return;
                        }
                    }
                }
            }
        }
    }
}

// fn cb(cs: *mut libc::c_char, lc:*mut linenoise::Completions ) {
//     let input: Option<&str>;
//     let ccurrent_input: std::c_str::CString;

//     unsafe {
//         ccurrent_input = c_str::CString::new(cs as *const _, false);
//         input = ccurrent_input.as_str();
//     }
//     match input {
//         None => { return; }
//         _ => {
//             let ip = input.unwrap();
//         }
//     }

//     unsafe {

//         let tep = "coucou".to_c_str();
//         let rep = tep.as_ptr();

//         let tap = "kiki".to_c_str();
//         let rap = tap.as_ptr();

//         let top = "koko".to_c_str();
//         let rop = top.as_ptr();

//         let mut vec = vec![rep, rap, rop];
//         (*lc).len = 1;
//         (*lc).cvec = vec.as_mut_ptr() as *mut *mut i8;
//     }

// }


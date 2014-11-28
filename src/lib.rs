use std::c_str;

pub mod ffi;


pub fn linenoise(prompt: &str) -> Option<String> {
    let cs = prompt.to_c_str().as_ptr();
    let mut retval:Option<String>;

    unsafe {
        let rret = ffi::linenoise(cs);

        let ptr = rret as *const i8;
        let ret = c_str::CString::new(ptr, false);
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

pub fn clear_screen() {
    unsafe {
        ffi::linenoiseClearScreen();
    }
}

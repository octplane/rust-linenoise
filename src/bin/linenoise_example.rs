extern crate linenoise;
extern crate libc;

use std::c_str;


fn cb(cs: *mut libc::c_char, lc:*mut linenoise::Completions ) {
	let input: Option<&str>;
	let ccurrent_input: std::c_str::CString;

	unsafe {
		ccurrent_input = c_str::CString::new(cs as *const _, true);
		input = ccurrent_input.as_str();
	}
	match input {
		None => { return; }
		_ => { println!("{}", input.unwrap()); }
	}
}

fn main() {
	println!("Hello !");
    loop {
	    let val = linenoise::linenoise("hello > ");
	    linenoise::set_completion_callback(cb);
        match val {
            None => { break }
            _ => {
                let input = val.unwrap();
                println!("> {}", input);
                linenoise::history_add(input.as_slice());
                if input.as_slice() == "clear" {
                	linenoise::clear_screen();
                }
            }
        }
    }
}
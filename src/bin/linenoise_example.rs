extern crate linenoise;
extern crate libc;

use std::c_str;

fn cb(cs: *mut libc::c_char, lc:*mut linenoise::Completions ) {
	let input: Option<&str>;
	let ccurrent_input: std::c_str::CString;

	unsafe {
		ccurrent_input = c_str::CString::new(cs as *const _, false);
		input = ccurrent_input.as_str();
	}
	match input {
		None => { return; }
		_ => {
			let ip = input.unwrap();
		}
	}

	let rep = "coucou".to_c_str();
	let adr = rep.as_ptr();

	let mut vec = Vec::new();

	vec.push(adr);

	unsafe {
		(*lc).len = 1;
		(*lc).cvec = vec.as_mut_ptr() as *mut *mut i8;
	}

}

fn main() {
	println!("Youhou.");
    linenoise::set_completion_callback(cb);
    loop {
	    let val = linenoise::linenoise("hello > ");
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
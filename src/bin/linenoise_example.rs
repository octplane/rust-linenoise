extern crate linenoise;
extern crate libc;

use std::c_str;

struct Completion {
	completions: *mut *mut i8,
	cb: *mut fn(&str) -> Vec<&str>
}

impl Completion {
	fn input(&self, prompt: &str) -> Option<String> {

	    return linenoise::linenoise(prompt);
	}
	fn set_callback(&mut self, callback:fn(&str) -> Vec<&str>) {
		self.cb = callback as *mut _;
	    linenoise::set_completion_callback(cb);
	}
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
				println!("Input: {}", ip);
			}
		}
	}
}

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

	unsafe {

		let tep = "coucou".to_c_str();
		let rep = tep.as_ptr();

		let tap = "kiki".to_c_str();
		let rap = tap.as_ptr();

		let top = "koko".to_c_str();
		let rop = top.as_ptr();

		let mut vec = vec![rep, rap, rop];
		(*lc).len = 1;
		(*lc).cvec = vec.as_mut_ptr() as *mut *mut i8;
	}

}

fn main() {
	println!("Youhou.");
	let completion = Completion{ completions: 0 as *mut *mut i8, cb: 0 as *mut fn(&str) -> Vec<&str>  };

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
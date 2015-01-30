#![feature(collections, core)]

extern crate linenoise;

fn callback(input: &str) -> Vec<String> {
	let mut ret : Vec<&str>;
	 if input.starts_with("s") {
		ret = vec!["suggestion", "suggestion2", "suggestion-three"];
	} else {
		ret = vec!["wot"];
	}

    return ret.iter().map(|s| s.to_string()).collect();
}

fn main() {
	linenoise::set_callback(callback);

    loop {
	    let val = linenoise::input("hello > ");
        match val {
            None => { break }
            Some(input) => {
                println!("> {}", input);
                linenoise::history_add(input.as_slice());
                if input.as_slice() == "clear" {
                	linenoise::clear_screen();
                }
            }
        }
    }
}

extern crate linenoise;
extern crate libc;

use linenoise::LINENOISE;


fn callback(input: &str) -> Vec<&str> {
	let ret = vec!["suggestion!", "suggestion2", "suggestion-three"];
	return ret;
}




fn main() {
	println!("Youhou.");
	LINENOISE.init(callback);

    loop {
	    let val = LINENOISE.input("hello > ");
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
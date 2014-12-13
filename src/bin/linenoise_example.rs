extern crate linenoise;
extern crate libc;

fn callback(input: &str) -> Vec<&str> {
	let ret = vec!["suggestion", "suggestion2", "suggestion-three"];
	return ret;
}


fn main() {
	linenoise::init(callback);

    loop {
	    let val = linenoise::input("hello > ");
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
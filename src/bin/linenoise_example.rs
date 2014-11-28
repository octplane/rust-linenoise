extern crate linenoise;

fn main() {
    loop {
    	linenoise::history_set_max_len(1);
	    let val = linenoise::linenoise(">>> ");
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